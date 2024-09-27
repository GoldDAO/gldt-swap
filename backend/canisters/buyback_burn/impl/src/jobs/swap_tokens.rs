use crate::state::{ mutate_state, read_state };
use crate::utils::{ calculate_percentage_of_amount, get_token_balance, RETRY_DELAY };
use crate::types::{ SwapClientEnum, TokenSwap, SwapClient };
use utils::rand::generate_random_delay;
use canister_time::run_now_then_interval;
use canister_tracing_macros::trace;
use icrc_ledger_types::icrc1::transfer::TransferArg;
use tracing::{ debug, info, error };
use anyhow::{ Result, anyhow };
use utils::env::Environment;
use utils::retry_async::retry_with_attempts;
use futures::future::join_all;

use canister_time::NANOS_PER_MILLISECOND;
const MAX_ATTEMPTS: u8 = 1;

pub const MEMO_SWAP: [u8; 7] = [0x4f, 0x43, 0x5f, 0x53, 0x57, 0x41, 0x50]; // OC_SWAP

pub fn start_job() {
    let buyback_burn_interval = read_state(|s| s.data.buyback_burn_interval);
    if read_state(|s| s.data.burn_config.validate_burn_rate()) {
        run_now_then_interval(buyback_burn_interval, run);
    } else {
        error!("Burn rate is invalid. The job wouldn't start");
    }
}

pub fn run() {
    ic_cdk::spawn(run_async_with_rand_delay());
}

pub fn run_now() {
    ic_cdk::spawn(run_async());
}

#[trace]
async fn run_async_with_rand_delay() {
    let buyback_burn_interval = read_state(|s| s.data.buyback_burn_interval);

    match generate_random_delay(buyback_burn_interval).await {
        Ok(random_delay) => {
            ic_cdk_timers::set_timer(random_delay, || ic_cdk::spawn(run_async()));
        }
        Err(e) => {
            error!("Failed to generate random delay: {}", e);
        }
    }
}

#[trace]
async fn run_async() {
    let swap_clients_iter = read_state(|state| state.data.swap_clients.into_iter());
    let mut token_swap_ids = Vec::new();

    // TODO: check that everything here is correct and get rid of clones
    let futures: Vec<_> = swap_clients_iter
        .map(|swap_client| {
            let args = swap_client.get_config();
            let token_swap = mutate_state(|state|
                state.data.token_swaps.push_new(args, state.env.now())
            );

            token_swap_ids.push(token_swap.swap_id);

            async {
                retry_with_attempts(MAX_ATTEMPTS, RETRY_DELAY, move || {
                    let swap_client = swap_client.clone();
                    let token_swap = token_swap.clone();
                    async move { process_token_swap(swap_client, token_swap).await }
                }).await
            }
        })
        .collect();

    let results = join_all(futures).await;

    let error_messages: Vec<anyhow::Error> = results.into_iter().filter_map(Result::err).collect();

    if error_messages.is_empty() {
        info!("Successfully processed all token swaps");
        for token_swap_id in token_swap_ids {
            let _ = mutate_state(|state| state.data.token_swaps.archive_swap(token_swap_id));
        }

        crate::jobs::burn_tokens::run();
    } else {
        error!("Failed to process some token swaps:\n{:?}", error_messages);
    }
}

pub(crate) async fn process_token_swap(
    swap_client: SwapClientEnum,
    mut token_swap: TokenSwap
) -> Result<()> {
    let burn_config = read_state(|s| s.data.burn_config.clone());
    let swap_config = swap_client.get_config();

    let min_output_amount = 0;
    let available_amount = get_token_balance(swap_config.input_token.ledger_id).await?;

    let input_amount = calculate_percentage_of_amount(available_amount, burn_config.burn_rate);
    debug!("input_amount: {}", input_amount);
    let amount_to_dex = input_amount.saturating_sub(swap_config.input_token.fee.into());
    debug!("amount_to_dex: {}", amount_to_dex);

    // Get the quote to decide whether swap or not
    let quote = match
        swap_client.get_quote(
            amount_to_dex.saturating_sub(swap_config.input_token.fee.into()),
            min_output_amount
        ).await
    {
        Ok(quote) => {
            match quote {
                Ok(q) => q,
                Err(error) => {
                    let msg = format!("{error:?}");
                    error!("Failed to get the quote: {}", msg.as_str());
                    return Err(anyhow!(msg));
                }
            }
        }
        Err(error) => {
            let msg = format!("{error:?}");
            error!("Failed to get the quote: {}", msg.as_str());
            return Err(anyhow!(msg));
        }
    };

    // NOTE: check if it makes sense to make swap (especially if there would be enough balance after the swap)
    if quote < burn_config.get_min_after_swap_amount() + (swap_config.output_token.fee as u128) {
        return Err(anyhow!("Insufficient balance to swap"));
    }

    // Get the deposit account
    let account = if let Some(a) = extract_result(&token_swap.deposit_account) {
        *a
    } else {
        match swap_client.deposit_account().await {
            Ok(a) => {
                mutate_state(|state| {
                    token_swap.deposit_account = Some(Ok(a));
                    state.data.token_swaps.upsert(token_swap.clone());
                });
                a
            }
            Err(error) => {
                let msg = format!("{error:?}");
                mutate_state(|state| {
                    token_swap.deposit_account = Some(Err(msg.clone()));
                    token_swap.success = Some(false);
                    state.data.token_swaps.upsert(token_swap);
                });
                error!("Failed to deposit tokens while swap: {}", msg.as_str());
                return Err(anyhow!(msg));
            }
        }
    };

    // Deposit tokens to the deposit account
    if extract_result(&token_swap.transfer).is_none() {
        let now = read_state(|state| state.env.now());
        let transfer_result = match
            icrc_ledger_canister_c2c_client::icrc1_transfer(
                swap_config.input_token.ledger_id,
                &(TransferArg {
                    from_subaccount: None,
                    to: account,
                    fee: Some(swap_config.input_token.fee.into()),
                    created_at_time: Some(now * NANOS_PER_MILLISECOND),
                    memo: Some(MEMO_SWAP.to_vec().into()),
                    amount: amount_to_dex.into(),
                })
            ).await
        {
            Ok(Ok(index)) => Ok(index),
            Ok(Err(error)) => Err(format!("{error:?}")),
            Err(error) => Err(format!("{error:?}")),
        };

        match transfer_result {
            Ok(index) => {
                mutate_state(|state| {
                    token_swap.transfer = Some(Ok(index.0.try_into().unwrap()));
                    state.data.token_swaps.upsert(token_swap.clone());
                });
            }
            Err(msg) => {
                mutate_state(|state| {
                    token_swap.transfer = Some(Err(msg.clone()));
                    token_swap.success = Some(false);
                    state.data.token_swaps.upsert(token_swap);
                });
                error!("Failed to transfer tokens: {}", msg.as_str());
                return Err(anyhow!(msg));
            }
        }
    }

    // Notify DEX
    if extract_result(&token_swap.notified_dex_at).is_none() {
        if let Err(error) = swap_client.deposit(amount_to_dex).await {
            let msg = format!("{error:?}");
            mutate_state(|state| {
                token_swap.notified_dex_at = Some(Err(msg.clone()));
                state.data.token_swaps.upsert(token_swap.clone());
            });
            error!("Failed to deposit tokens: {}", msg.as_str());
            return Err(anyhow!(msg));
        } else {
            mutate_state(|state| {
                token_swap.notified_dex_at = Some(Ok(()));
                state.data.token_swaps.upsert(token_swap.clone());
            });
        }
    }

    // Swap the tokens
    let swap_result = if let Some(a) = extract_result(&token_swap.amount_swapped).cloned() {
        a
    } else {
        match
            swap_client.swap(
                amount_to_dex.saturating_sub(swap_config.input_token.fee.into()),
                min_output_amount
            ).await
        {
            Ok(a) => {
                mutate_state(|state| {
                    token_swap.amount_swapped = Some(Ok(a.clone()));
                    state.data.token_swaps.upsert(token_swap.clone());
                });
                a
            }
            Err(error) => {
                let msg = format!("{error:?}");
                mutate_state(|state| {
                    token_swap.amount_swapped = Some(Err(msg.clone()));
                    state.data.token_swaps.upsert(token_swap.clone());
                });
                error!("Failed to swap tokens: {}", msg.as_str());
                return Err(anyhow!(msg));
            }
        }
    };

    let (successful_swap, amount_out) = if let Ok(amount_swapped) = swap_result {
        (true, amount_swapped.saturating_sub(swap_config.output_token.fee.into()))
    } else {
        (false, amount_to_dex.saturating_sub(swap_config.input_token.fee.into()))
    };

    // Withdraw tokens from the DEX
    if extract_result(&token_swap.withdrawn_from_dex_at).is_none() {
        if let Err(error) = swap_client.withdraw(successful_swap, amount_out).await {
            let msg = format!("{error:?}");
            mutate_state(|state| {
                token_swap.withdrawn_from_dex_at = Some(Err(msg.clone()));
                state.data.token_swaps.upsert(token_swap.clone());
            });
            error!("Failed to withdraw tokens: {}", msg.as_str());
            return Err(anyhow!(msg));
        } else {
            mutate_state(|state| {
                token_swap.withdrawn_from_dex_at = Some(Ok(amount_out));
                token_swap.success = Some(successful_swap);
                state.data.token_swaps.upsert(token_swap);
            });
        }
    }

    if successful_swap {
        Ok(())
    } else {
        Err(anyhow!("The swap failed"))
    }
}

fn extract_result<T>(subtask: &Option<Result<T, String>>) -> Option<&T> {
    subtask.as_ref().and_then(|t| t.as_ref().ok())
}