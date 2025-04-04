use crate::guards::caller_is_governance_principal;
use crate::state::{read_state, RuntimeState};
use candid::Principal;
use canister_tracing_macros::trace;
use ic_cdk::{query, update};
pub use icp_neuron_api_canister::stake_nns_neuron::StakeNnsNeuronResponse;
use icrc_ledger_types::icrc1::account::Account;
use icrc_ledger_types::icrc1::transfer::TransferArg;
use ledger_utils::compute_neuron_staking_subaccount_bytes;
use nns_governance_canister::types::manage_neuron::claim_or_refresh::{By, MemoAndController};
use nns_governance_canister::types::manage_neuron::{ClaimOrRefresh, Command};
use nns_governance_canister::types::{manage_neuron_response, ManageNeuron};
use tracing::{error, info};
use types::CanisterId;
use utils::rand::generate_rand_nonce;

#[query(guard = "caller_is_governance_principal", hidden = true)]
#[trace]
async fn stake_nns_neuron_validate() -> Result<String, String> {
    Ok("No arguments to validate".to_string())
}

#[update(guard = "caller_is_governance_principal")]
#[trace]
async fn stake_nns_neuron() -> StakeNnsNeuronResponse {
    match stake_nns_neuron_impl().await {
        Ok(neuron_id) => {
            info!(neuron_id, "Created new neuron.");
            StakeNnsNeuronResponse::Success(neuron_id)
        }
        Err(error) => {
            error!(error);
            StakeNnsNeuronResponse::InternalError(error)
        }
    }
}

async fn stake_nns_neuron_impl() -> Result<u64, String> {
    let nonce = generate_rand_nonce().await?;

    let PrepareResult {
        nns_governance_canister_id,
        icp_ledger_canister_id,
        principal,
    } = read_state(prepare)?;

    let subaccount = compute_neuron_staking_subaccount_bytes(principal, nonce);

    match icrc_ledger_canister_c2c_client::icrc1_transfer(
        icp_ledger_canister_id,
        &(TransferArg {
            from_subaccount: None,
            to: Account {
                owner: nns_governance_canister_id,
                subaccount: Some(subaccount),
            },
            fee: Some((10_000u32).into()),
            created_at_time: None,
            memo: Some(nonce.into()),
            amount: (100_000_000u32).into(), // initialised with 1 ICP, further can be added afterwards
        }),
    )
    .await
    {
        Ok(Ok(_)) => {}
        Ok(Err(error)) => {
            return Err(format!("Transfer error: {error:?}"));
        }
        Err(error) => {
            return Err(format!("Network error: {error:?}"));
        }
    }

    match nns_governance_canister_c2c_client::manage_neuron(
        nns_governance_canister_id,
        &(ManageNeuron {
            id: None,
            neuron_id_or_subaccount: None,
            command: Some(Command::ClaimOrRefresh(ClaimOrRefresh {
                by: Some(By::MemoAndController(MemoAndController {
                    controller: Some(principal),
                    memo: nonce,
                })),
            })),
        }),
    )
    .await
    {
        Ok(response) => match response.command {
            Some(manage_neuron_response::Command::ClaimOrRefresh(c)) => {
                let neuron_id = c.refreshed_neuron_id.unwrap().id;
                Ok(neuron_id)
            }
            response => {
                return Err(format!("Governance error: {response:?}"));
            }
        },
        Err(error) => Err(format!("Network error: {error:?}")),
    }
}

struct PrepareResult {
    nns_governance_canister_id: CanisterId,
    icp_ledger_canister_id: CanisterId,
    principal: Principal,
}

fn prepare(state: &RuntimeState) -> Result<PrepareResult, String> {
    Ok(PrepareResult {
        nns_governance_canister_id: state.data.nns_governance_canister_id,
        icp_ledger_canister_id: state.data.icp_ledger_canister_id,
        principal: state.data.get_principal()?,
    })
}
