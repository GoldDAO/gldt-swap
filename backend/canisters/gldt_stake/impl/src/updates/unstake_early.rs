use candid::{Nat, Principal};
use canister_time::timestamp_millis;
use canister_tracing_macros::trace;
pub use gldt_stake_api_canister::unstake_early::{
    Args as UnstakeEarlyArgs, Response as UnstakeEarlyResponse,
};
use gldt_stake_common::stake_position::{
    DissolveState, StakePosition, StakePositionId, UnstakeEarlyRequestErrors,
};
use gldt_stake_common::stake_position_event::UnstakeState;
use gldt_stake_common::{ledgers::GLDT_TX_FEE, stake_position_event::UnstakeEarlyStatus};
use icrc_ledger_canister_c2c_client::icrc1_transfer;
use icrc_ledger_types::icrc1::account::Account;
use icrc_ledger_types::icrc1::transfer::TransferArg;
use tracing::error;

use crate::guards::GuardPrincipal;
use crate::model::archive_system::archive_stake_position;
use crate::utils::{commit_changes, set_unstake_state_of_position};
use crate::{
    guards::reject_anonymous_caller,
    state::{mutate_state, read_state},
};
use ic_cdk::{caller, update};

#[update]
#[trace]
async fn unstake_early(position_id: UnstakeEarlyArgs) -> UnstakeEarlyResponse {
    unstake_early_impl(position_id).await
}

async fn unstake_early_impl(position_id: UnstakeEarlyArgs) -> UnstakeEarlyResponse {
    // 1. check user isn't anon
    let caller = caller();
    reject_anonymous_caller().map_err(UnstakeEarlyRequestErrors::InvalidPrincipal)?;
    let _guard_principal =
        GuardPrincipal::new(caller).map_err(UnstakeEarlyRequestErrors::AlreadyProcessing)?;

    // find the position
    let position = read_state(|s| s.data.stake_system.get_stake_position(position_id)).ok_or(
        UnstakeEarlyRequestErrors::NotFound(format!(
            "Cant find active stake position with ID : {position_id}"
        )),
    )?;

    if position.owned_by != caller {
        return Err(UnstakeEarlyRequestErrors::NotAuthorized(
            "You do not have permission to unstake this stake position early".to_string(),
        ));
    }

    position
        .can_unstake_early()
        .map_err(UnstakeEarlyRequestErrors::UnstakeErrors)?;

    let early_unstake_fee = position.calculate_unstake_early_fee();
    let position_stake = position.staked.clone();
    let amount_to_unstake = position.staked.clone() - early_unstake_fee.clone();
    let amount_for_user = amount_to_unstake - GLDT_TX_FEE;

    set_unstake_state_of_position(
        &position_id,
        &position,
        UnstakeState::EarlyUnstake(UnstakeEarlyStatus::InProgress),
    );
    commit_changes().await;

    let stake_position = transfer_stake_to_user(
        amount_for_user,
        caller,
        position_id,
        position,
        early_unstake_fee,
        position_stake,
    )
    .await?;

    let position_id_to_archive = position_id;
    let position_to_archive = stake_position.clone();
    ic_cdk::spawn(async move {
        let _ = archive_stake_position(position_id_to_archive, position_to_archive).await;
    });

    Ok((stake_position, timestamp_millis(), position_id).into())
}

async fn transfer_stake_to_user(
    amount_for_user: Nat,
    caller: Principal,
    position_id: StakePositionId,
    position: StakePosition,
    early_unstake_fee: Nat,
    position_stake: Nat,
) -> Result<StakePosition, UnstakeEarlyRequestErrors> {
    let gldt_ledger = read_state(|s| s.data.gldt_ledger_id);
    let transfer_args = TransferArg {
        from_subaccount: None,
        to: Account {
            owner: caller,
            subaccount: None,
        },
        fee: None,
        created_at_time: None,
        memo: None,
        amount: amount_for_user,
    };

    match icrc1_transfer(gldt_ledger, &transfer_args).await {
        Ok(Ok(_)) => {
            set_unstake_state_of_position(
                &position_id,
                &position,
                UnstakeState::EarlyUnstake(UnstakeEarlyStatus::UnstakedEarly),
            );
            let mut updated_position = position.clone();
            updated_position.dissolve_state = DissolveState::Dissolved;
            updated_position.dissolved_date = Some(timestamp_millis());
            updated_position.staked = Nat::from(0u64);
            mutate_state(|s| {
                s.data
                    .stake_system
                    .update_stake_position(&position_id, updated_position.clone());
                s.data.stake_system.pending_fee_transfer_amount += early_unstake_fee;
                s.data.stake_system.total_staked -= position_stake;
            });
            Ok(updated_position)
        }
        Ok(Err(e)) => {
            error!(
                "UNSTAKE EARLY :: Failed :: position id - {} transfer error - {:?}. transfer args - {:?}",
                position_id, e, &transfer_args
            );
            set_unstake_state_of_position(
                &position_id,
                &position,
                UnstakeState::EarlyUnstake(UnstakeEarlyStatus::Failed(format!("{e:?}"))),
            );
            Err(UnstakeEarlyRequestErrors::TransferError(format!("{e:?}")))
        }
        Err(e) => {
            error!(
                "UNSTAKE EARLY :: Failed :: position id - {} call error - {:?}. transfer args - {:?}",
                position_id, e, &transfer_args
            );
            set_unstake_state_of_position(
                &position_id,
                &position,
                UnstakeState::EarlyUnstake(UnstakeEarlyStatus::Failed(format!("{e:?}"))),
            );
            Err(UnstakeEarlyRequestErrors::CallError(format!("{e:?}")))
        }
    }
}
