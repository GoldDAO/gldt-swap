use crate::state::mutate_state;
use crate::state::read_state;
use crate::State;
use canister_time::run_now_then_interval;
use canister_tracing_macros::trace;
use ic_ledger_types::{
    AccountIdentifier, BlockIndex, Memo, Subaccount, Timestamp, Tokens, TransferArgs,
};
use std::time::Duration;
use tracing::{error, info};
use types::{CanisterId, TimestampMillis};
use utils::env::Environment;

const INTERVAL: Duration = Duration::from_secs(10 * 60); // 10 minutes
const MEMO_TOP_UP_CANISTER: Memo = Memo(0x50555054); // == 'TPUP'

pub fn start_job() {
    run_now_then_interval(INTERVAL, run);
}

fn run() {
    match read_state(get_next_action) {
        Action::BurnIcp(burn) => ic_cdk::spawn(burn_icp(burn)),
        Action::NotifyTopUp(notify) => ic_cdk::spawn(notify_cmc(notify)),
        Action::None => {}
    }
}

enum Action {
    BurnIcp(BurnIcpArgs),
    NotifyTopUp(NotifyTopUpDetails),
    None,
}

#[derive(Debug)]
struct BurnIcpArgs {
    amount: Tokens,
    this_canister_id: CanisterId,
    ledger: CanisterId,
    cmc: CanisterId,
    now: TimestampMillis,
}

#[derive(Debug)]
struct NotifyTopUpDetails {
    this_canister_id: CanisterId,
    cmc: CanisterId,
    block_index: BlockIndex,
}

fn get_next_action(state: &State) -> Action {
    if let Some(block_index) = state.data.burn_config.cycles_top_up_pending_notification {
        Action::NotifyTopUp(NotifyTopUpDetails {
            this_canister_id: state.env.canister_id(),
            cmc: state.data.burn_config.cycles_minting_canister,
            block_index,
        })
    } else {
        let cycles_balance = state.env.cycles_balance();

        // NOTE: here we make sure that it would be enough funds to top up all the canisters. min_cycles_balance * X, where X - amount of canisters to top up + 1 (to save some cycles for the cycles manager itself)
        // TODO: it would be better to store the canisters to top up quantity in the state, but it should always be relevant and updated on time, which seems to be not reachable.
        let monitored_canisters_quantity: u64 = state
            .data
            .canisters
            .get_canisters_quantity()
            .try_into()
            .unwrap();
        if cycles_balance
            < (monitored_canisters_quantity + 1) * state.data.top_up_config.max_top_up_amount
        {
            Action::BurnIcp(BurnIcpArgs {
                amount: state.data.burn_config.icp_burn_amount,
                this_canister_id: state.env.canister_id(),
                ledger: state.data.burn_config.icp_ledger_canister,
                cmc: state.data.burn_config.cycles_minting_canister,
                now: state.env.now(),
            })
        } else {
            Action::None
        }
    }
}

#[trace]
async fn burn_icp(burn_args: BurnIcpArgs) {
    info!(%burn_args.amount, "Burning ICP into cycles");

    match icp_ledger_canister_c2c_client::transfer(
        burn_args.ledger,
        &TransferArgs {
            memo: MEMO_TOP_UP_CANISTER,
            amount: burn_args.amount,
            fee: ic_ledger_types::DEFAULT_FEE,
            from_subaccount: None,
            to: AccountIdentifier::new(
                &burn_args.cmc,
                &Subaccount::from(burn_args.this_canister_id),
            ),
            created_at_time: Some(Timestamp {
                timestamp_nanos: burn_args.now * 1_000_000,
            }),
        },
    )
    .await
    {
        Ok(Ok(block_index)) => {
            info!(block_index, "Transferred ICP to CMC");
            notify_cmc(NotifyTopUpDetails {
                this_canister_id: burn_args.this_canister_id,
                cmc: burn_args.cmc,
                block_index,
            })
            .await;
        }
        Ok(Err(err)) => {
            error!(?err, "Failed to burn ICP into cycles");
        }
        Err((code, message)) => {
            error!(?code, message, "Failed to burn ICP into cycles");
        }
    }
}

#[trace]
async fn notify_cmc(notify_details: NotifyTopUpDetails) {
    let response = cycles_minting_canister_c2c_client::notify_top_up(
        notify_details.cmc,
        &cycles_minting_canister::notify_top_up::Args {
            block_index: notify_details.block_index,
            canister_id: notify_details.this_canister_id,
        },
    )
    .await;

    match response {
        Ok(Ok(cycles)) => {
            info!(cycles, "Canister topped up with cycles");
        }
        // NOTE: here an error could occure: Failed to notify the CMC Some(Refunded { reason: "More than 50_000_000_000_000_000 cycles have been minted in the last 3600 seconds, please try again later." }).
        err => {
            error!(?err, "Failed to notify the CMC");
            mutate_state(|state| {
                state.data.burn_config.cycles_top_up_pending_notification =
                    Some(notify_details.block_index)
            });
        }
    }
}
