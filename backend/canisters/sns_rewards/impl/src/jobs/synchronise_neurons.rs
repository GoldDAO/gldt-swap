/*!
# SNS neuron maturity process

This job is responsible for processing the maturity of neurons. It is run every
epoch and processes the maturity of all neurons in this epoch. This maturity
is stored in the canister and is used to determine the rewards that a neuron
is eligible for.
*/

use canister_time::{
    now_millis, run_interval, start_job_daily_at, timestamp_millis, DAY_IN_MS, HOUR_IN_MS,
};
use sns_governance_canister::types::{Neuron, NeuronId};
use std::{
    collections::{btree_map, HashMap},
    time::Duration,
};
use tracing::{debug, error, info, warn};
use types::{Maturity, Milliseconds, NeuronInfo};

use crate::{
    state::{mutate_state, read_state, RuntimeState},
    utils::tracer,
};

pub fn start_job() {
    start_job_daily_at(9, run);
}

pub fn run() {
    ic_cdk::spawn(run_async());
}

async fn run_async() {
    synchronise_neuron_data().await;
}

pub async fn synchronise_neuron_data() {
    let is_synchronizing_neurons = read_state(|s| s.data.is_synchronizing_neurons);
    if is_synchronizing_neurons {
        return;
    }
    let canister_id = read_state(|state| state.data.sns_governance_canister);
    let is_test_mode = read_state(|s| s.env.is_test_mode());
    mutate_state(|state| {
        state.data.sync_info.last_synced_start = timestamp_millis();
        state.set_is_synchronizing_neurons(false);
    });

    let mut number_of_scanned_neurons = 0;
    let mut continue_scanning = true;
    // the max limit of 100 is given by the list_neurons call implementation. Cannot increase it.
    let limit = 100;

    let mut args = sns_governance_canister::list_neurons::Args {
        limit,
        start_page_at: None,
        of_principal: None,
    };

    while continue_scanning {
        continue_scanning = false;

        debug!("Fetching neuron data");
        match sns_governance_canister_c2c_client::list_neurons(canister_id, &args).await {
            Ok(response) => {
                mutate_state(|state| {
                    debug!("Updating neurons");
                    response.neurons.iter().for_each(|neuron| {
                        update_neuron_maturity(state, neuron);
                    });
                });
                let number_of_received_neurons = response.neurons.len();
                if (number_of_received_neurons as u32) == limit {
                    args.start_page_at = response.neurons.last().map_or_else(
                        || {
                            error!(
                                "Missing last neuron to continue iterating.
                                This should not be possible as the limits are checked. Stopping loop here."
                            );
                            None
                        },
                        |n| {
                            continue_scanning = true;
                            if is_test_mode && number_of_scanned_neurons == 400 {
                                continue_scanning = false;
                            }
                            n.id.clone()
                        }
                    );
                }
                number_of_scanned_neurons += number_of_received_neurons;
            }
            Err(err) => {
                let error_message = format!("{err:?}");
                error!(?error_message, "Error fetching neuron data");
            }
        }
    }
    info!("Successfully scanned {number_of_scanned_neurons} neurons.");
    mutate_state(|state| {
        state.data.sync_info.last_synced_end = timestamp_millis();
        state.data.sync_info.last_synced_number_of_neurons = number_of_scanned_neurons;
        state.set_is_synchronizing_neurons(false);
    });
}

// Function to update neuron maturity
fn update_neuron_maturity(state: &mut RuntimeState, neuron: &Neuron) {
    // This function only returns Some() if the neuron is initialised or its maturity has changed
    if let Some(id) = &neuron.id {
        let updated_neuron: Option<(NeuronId, NeuronInfo)>;

        let maturity = calculate_total_maturity(neuron);

        let neuron_info = NeuronInfo {
            last_synced_maturity: maturity,
            accumulated_maturity: 0,
            rewarded_maturity: HashMap::new(),
            last_disburse_event_considered: Some(0),
        };

        // TODO - check age of neuron to avoid someone gaming the system by spawning neurons (check if really relevant)
        match state.data.neuron_maturity.entry(id.clone()) {
            btree_map::Entry::Vacant(entry) => {
                entry.insert(neuron_info.clone());
                updated_neuron = Some((id.clone(), neuron_info));
            }
            btree_map::Entry::Occupied(mut entry) => {
                let neuron_info_entry = entry.get_mut();
                let spawning_maturity =
                    extract_and_sum_up_new_disburse_events(neuron, neuron_info_entry);
                println!("spawning_maturity {spawning_maturity}");
                let total_maturity = maturity + spawning_maturity;
                println!("total_maturity {total_maturity}");

                if let Some(delta) =
                    total_maturity.checked_sub(neuron_info_entry.last_synced_maturity)
                {
                    // only add the difference if the maturity has increased
                    if delta == 0 {
                        return;
                    }
                    // update accumulated maturity
                    neuron_info_entry.accumulated_maturity = neuron_info_entry
                        .accumulated_maturity
                        .checked_add(delta)
                        .unwrap_or(neuron_info_entry.accumulated_maturity);
                }
                // update the last_synced_maturity
                neuron_info_entry.last_synced_maturity = maturity;
                updated_neuron = Some((id.clone(), neuron_info_entry.clone()));
            }
        }
        // update history
        if let Some((n_id, n_info)) = updated_neuron {
            state
                .data
                .maturity_history
                .insert((n_id, state.data.sync_info.last_synced_start), n_info)
        }
    }
}

// Function to update principal-neuron mapping
fn extract_and_sum_up_new_disburse_events(neuron: &Neuron, neuron_info: &mut NeuronInfo) -> u64 {
    let mut last_disburse_event_considered =
        neuron_info.last_disburse_event_considered.unwrap_or(0);
    let total_from_new_disburse_events =
        neuron
            .disburse_maturity_in_progress
            .iter()
            .fold(0u64, |mut acc, event| {
                if event.timestamp_of_disbursement_seconds
                    > neuron_info.last_disburse_event_considered.unwrap_or(0)
                {
                    acc += event.amount_e8s;
                }
                if event.timestamp_of_disbursement_seconds > last_disburse_event_considered {
                    last_disburse_event_considered = event.timestamp_of_disbursement_seconds;
                }
                acc
            });

    neuron_info.last_disburse_event_considered = Some(last_disburse_event_considered);
    println!("total_from_new_disburse_events {total_from_new_disburse_events}");
    total_from_new_disburse_events
}

fn calculate_total_maturity(neuron: &Neuron) -> Maturity {
    neuron
        .maturity_e8s_equivalent
        .checked_add(neuron.staked_maturity_e8s_equivalent.unwrap_or(0))
        .unwrap_or_else(|| {
            let id = neuron.id.clone().unwrap_or_default();
            warn!("Unexpected overflow when calculating total maturity of neuron {id}");
            0
        })
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use sns_governance_canister::types::{DisburseMaturityInProgress, Neuron, NeuronId};
    use types::NeuronInfo;

    use crate::state::{init_state, mutate_state, read_state, RuntimeState};

    use super::update_neuron_maturity;

    fn init_runtime_state() {
        init_state(RuntimeState::default());
    }

    #[test]
    fn test_insert_update_neuron() {
        init_runtime_state();

        let neuron_id =
            NeuronId::new("2a9ab729b173e14cc88c6c4d7f7e9f3e7468e72fc2b49f76a6d4f5af37397f98")
                .unwrap();
        let limit = 5;

        let mut neuron = Neuron::default();
        neuron.id = Some(neuron_id.clone());

        // ********************************
        // 1. Insert new neuron
        // ********************************

        mutate_state(|state| {
            update_neuron_maturity(state, &neuron);
        });

        let mut expected_result = NeuronInfo {
            accumulated_maturity: 0,
            last_synced_maturity: 0,
            rewarded_maturity: HashMap::new(),
            last_disburse_event_considered: Some(0),
        };
        let mut result =
            read_state(|state| state.data.neuron_maturity.get(&neuron_id).cloned()).unwrap();

        assert_eq!(result, expected_result);

        let mut expected_result_history = vec![(0, expected_result)];
        let mut result_history = read_state(|state| {
            state
                .data
                .maturity_history
                .get_maturity_history(neuron_id.clone(), limit)
        });

        assert_eq!(result_history, expected_result_history);

        // ********************************
        // 2. Increase neuron maturity
        // ********************************

        neuron.maturity_e8s_equivalent = 100;
        neuron.staked_maturity_e8s_equivalent = Some(50);

        mutate_state(|state| {
            state.data.sync_info.last_synced_start += 100;
            update_neuron_maturity(state, &neuron);
        });

        expected_result = NeuronInfo {
            accumulated_maturity: 150,
            last_synced_maturity: 150,
            rewarded_maturity: HashMap::new(),
            last_disburse_event_considered: Some(0),
        };
        result = read_state(|state| state.data.neuron_maturity.get(&neuron_id).cloned()).unwrap();

        assert_eq!(result, expected_result);

        expected_result_history.push((100, expected_result));
        result_history = read_state(|state| {
            state
                .data
                .maturity_history
                .get_maturity_history(neuron_id.clone(), limit)
        });

        assert_eq!(result_history, expected_result_history);

        // ********************************
        // 3. Reduce neuron maturity
        // ********************************

        neuron.maturity_e8s_equivalent = 0;
        neuron.staked_maturity_e8s_equivalent = Some(50);

        mutate_state(|state| {
            state.data.sync_info.last_synced_start += 150;
            update_neuron_maturity(state, &neuron);
        });

        expected_result = NeuronInfo {
            accumulated_maturity: 150,
            last_synced_maturity: 50,
            rewarded_maturity: HashMap::new(),
            last_disburse_event_considered: Some(0),
        };
        result = read_state(|state| state.data.neuron_maturity.get(&neuron_id).cloned()).unwrap();

        assert_eq!(result, expected_result);

        expected_result_history.push((250, expected_result));
        result_history = read_state(|state| {
            state
                .data
                .maturity_history
                .get_maturity_history(neuron_id.clone(), limit)
        });

        assert_eq!(result_history, expected_result_history);

        // ********************************
        // 4. No change in neuron maturity
        // ********************************

        neuron.maturity_e8s_equivalent = 0;
        neuron.staked_maturity_e8s_equivalent = Some(50);

        mutate_state(|state| {
            state.data.sync_info.last_synced_start += 150;
            update_neuron_maturity(state, &neuron);
        });

        expected_result = NeuronInfo {
            accumulated_maturity: 150,
            last_synced_maturity: 50,
            rewarded_maturity: HashMap::new(),
            last_disburse_event_considered: Some(0),
        };
        result = read_state(|state| state.data.neuron_maturity.get(&neuron_id).cloned()).unwrap();

        assert_eq!(result, expected_result);

        // `expected_result_history` stays the same
        result_history = read_state(|state| {
            state
                .data
                .maturity_history
                .get_maturity_history(neuron_id.clone(), limit)
        });

        assert_eq!(result_history, expected_result_history);
    }

    #[test]
    fn test_neuron_with_disburse_event() {
        init_runtime_state();

        let neuron_id =
            NeuronId::new("2a9ab729b173e14cc88c6c4d7f7e9f3e7468e72fc2b49f76a6d4f5af37397f98")
                .unwrap();
        let limit = 5;

        let mut neuron = Neuron::default();
        neuron.id = Some(neuron_id.clone());

        // ********************************
        // 1. Insert new neuron
        // ********************************

        mutate_state(|state| {
            update_neuron_maturity(state, &neuron);
        });

        let mut expected_result = NeuronInfo {
            accumulated_maturity: 0,
            last_synced_maturity: 0,
            rewarded_maturity: HashMap::new(),
            last_disburse_event_considered: Some(0),
        };
        let mut result =
            read_state(|state| state.data.neuron_maturity.get(&neuron_id).cloned()).unwrap();

        assert_eq!(result, expected_result);

        let mut expected_result_history = vec![(0, expected_result)];
        let mut result_history = read_state(|state| {
            state
                .data
                .maturity_history
                .get_maturity_history(neuron_id.clone(), limit)
        });

        assert_eq!(result_history, expected_result_history);

        // ********************************
        // 2. Increase neuron maturity
        // ********************************

        neuron.maturity_e8s_equivalent = 100;
        neuron.staked_maturity_e8s_equivalent = Some(50);

        mutate_state(|state| {
            state.data.sync_info.last_synced_start += 100;
            update_neuron_maturity(state, &neuron);
        });

        expected_result = NeuronInfo {
            accumulated_maturity: 150,
            last_synced_maturity: 150,
            rewarded_maturity: HashMap::new(),
            last_disburse_event_considered: Some(0),
        };
        result = read_state(|state| state.data.neuron_maturity.get(&neuron_id).cloned()).unwrap();

        assert_eq!(result, expected_result);

        expected_result_history.push((100, expected_result));
        result_history = read_state(|state| {
            state
                .data
                .maturity_history
                .get_maturity_history(neuron_id.clone(), limit)
        });

        assert_eq!(result_history, expected_result_history);

        // ********************************
        // 3. Reduce neuron maturity with a disburse event
        // ********************************

        neuron.maturity_e8s_equivalent = 0;
        neuron.staked_maturity_e8s_equivalent = None;
        neuron.disburse_maturity_in_progress = vec![DisburseMaturityInProgress {
            amount_e8s: 500u64,
            timestamp_of_disbursement_seconds: 10,
            account_to_disburse_to: None,
        }];

        mutate_state(|state| {
            state.data.sync_info.last_synced_start += 150;
            update_neuron_maturity(state, &neuron);
        });

        expected_result = NeuronInfo {
            accumulated_maturity: 500,
            last_synced_maturity: 0,
            rewarded_maturity: HashMap::new(),
            last_disburse_event_considered: Some(10),
        };
        result = read_state(|state| state.data.neuron_maturity.get(&neuron_id).cloned()).unwrap();

        assert_eq!(result, expected_result);

        // ********************************
        // 4. Reduce neuron maturity with a second disburse event but with the other still present as a record
        // ********************************

        neuron.maturity_e8s_equivalent = 0;
        neuron.staked_maturity_e8s_equivalent = None;
        neuron.disburse_maturity_in_progress = vec![
            DisburseMaturityInProgress {
                amount_e8s: 500u64,
                timestamp_of_disbursement_seconds: 10,
                account_to_disburse_to: None,
            },
            DisburseMaturityInProgress {
                amount_e8s: 400u64,
                timestamp_of_disbursement_seconds: 20,
                account_to_disburse_to: None,
            },
        ];

        mutate_state(|state| {
            state.data.sync_info.last_synced_start += 150;
            update_neuron_maturity(state, &neuron);
        });

        expected_result = NeuronInfo {
            accumulated_maturity: 900,
            last_synced_maturity: 0,
            rewarded_maturity: HashMap::new(),
            last_disburse_event_considered: Some(20),
        };
        result = read_state(|state| state.data.neuron_maturity.get(&neuron_id).cloned()).unwrap();

        assert_eq!(result, expected_result);

        // ********************************
        // 5. Keep the same disburse events but nothing extra should happen because they were already accounted for
        // ********************************

        neuron.maturity_e8s_equivalent = 0;
        neuron.staked_maturity_e8s_equivalent = None;
        neuron.disburse_maturity_in_progress = vec![
            DisburseMaturityInProgress {
                amount_e8s: 500u64,
                timestamp_of_disbursement_seconds: 10,
                account_to_disburse_to: None,
            },
            DisburseMaturityInProgress {
                amount_e8s: 400u64,
                timestamp_of_disbursement_seconds: 20,
                account_to_disburse_to: None,
            },
        ];

        mutate_state(|state| {
            state.data.sync_info.last_synced_start += 150;
            update_neuron_maturity(state, &neuron);
        });

        expected_result = NeuronInfo {
            accumulated_maturity: 900,
            last_synced_maturity: 0,
            rewarded_maturity: HashMap::new(),
            last_disburse_event_considered: Some(20),
        };
        result = read_state(|state| state.data.neuron_maturity.get(&neuron_id).cloned()).unwrap();

        assert_eq!(result, expected_result);

        // ********************************
        // 6. increase the normal maturity
        // ********************************

        neuron.maturity_e8s_equivalent = 50;
        neuron.staked_maturity_e8s_equivalent = None;
        neuron.disburse_maturity_in_progress = vec![
            DisburseMaturityInProgress {
                amount_e8s: 500u64,
                timestamp_of_disbursement_seconds: 10,
                account_to_disburse_to: None,
            },
            DisburseMaturityInProgress {
                amount_e8s: 400u64,
                timestamp_of_disbursement_seconds: 20,
                account_to_disburse_to: None,
            },
        ];

        mutate_state(|state| {
            state.data.sync_info.last_synced_start += 150;
            update_neuron_maturity(state, &neuron);
        });

        expected_result = NeuronInfo {
            accumulated_maturity: 950,
            last_synced_maturity: 50,
            rewarded_maturity: HashMap::new(),
            last_disburse_event_considered: Some(20),
        };
        result = read_state(|state| state.data.neuron_maturity.get(&neuron_id).cloned()).unwrap();

        assert_eq!(result, expected_result);
    }
}
