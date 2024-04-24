use std::time::Duration;

use candid::{ CandidType, Deserialize, Nat, Principal };
use canister_time::DAY_IN_MS;
use icrc_ledger_types::icrc1::account::Account;
use serde::Serialize;
use sns_governance_canister::types::NeuronId;
use sns_rewards::{ consts::REWARD_POOL_SUB_ACCOUNT, model::payment_processor::PaymentRound };
use types::TokenSymbol;

use crate::{
    client::{
        icrc1::client::{ balance_of, transfer },
        pocket::execute_update_multi_args,
        rewards::{ get_active_payment_rounds, get_neuron_by_id },
    },
    setup::{ default_test_setup, setup::setup_reward_pools },
    utils::tick_n_blocks,
};

#[derive(Deserialize, CandidType, Serialize)]
pub struct GetNeuronRequest {
    neuron_id: NeuronId,
}

#[test]
fn test_distribute_rewards_happy_path() {
    let mut test_env = default_test_setup();

    let icp_ledger_id = test_env.token_ledgers.get("icp_ledger_canister_id").unwrap().clone();
    let controller = test_env.controller;
    let rewards_canister_id = test_env.rewards_canister_id;

    let icp_token = TokenSymbol::parse("ICP").unwrap();
    let ogy_token = TokenSymbol::parse("OGY").unwrap();
    let gldgov_token = TokenSymbol::parse("GLDGov").unwrap();

    let neuron_id_1 = test_env.neuron_data.get(&0usize).unwrap().clone().id.unwrap();

    // ********************************
    // 1. Distribute rewards
    // ********************************

    test_env.simulate_neuron_voting(2);

    // TRIGGER - synchronize_neurons
    test_env.pic.advance_time(Duration::from_millis(DAY_IN_MS));
    tick_n_blocks(&test_env.pic, 100);

    // TRIGGER - distribute_rewards
    test_env.pic.advance_time(Duration::from_millis(DAY_IN_MS * 6));
    tick_n_blocks(&test_env.pic, 100);
    test_env.pic.advance_time(Duration::from_secs(60 * 5));
    tick_n_blocks(&test_env.pic, 100);

    // ********************************
    // 2. Check Neuron sub account got paid correctly
    // ********************************

    let fees = (test_env.neuron_data.len() as u64) * 10_000 + 10_000;
    let payment_round_pool_amount = (100_000_000_000u64 - fees) as f64;
    let total_maturity: f64 = ((test_env.neuron_data.len() as u64) * 100_000u64) as f64;
    let percentage = (100_000 as f64) / total_maturity;
    let expected_reward = (payment_round_pool_amount * percentage) as u64;
    assert_eq!(expected_reward, 9_999_989_000);

    let neuron_sub_account = Account {
        owner: rewards_canister_id,
        subaccount: Some(neuron_id_1.clone().into()),
    };
    let neuron_icp_balance = balance_of(&test_env.pic, icp_ledger_id, neuron_sub_account);
    assert_eq!(neuron_icp_balance, expected_reward);
    test_env.pic.tick();

    // ********************************
    // 3. Distribute rewards
    // ********************************

    test_env.simulate_neuron_voting(3);
    setup_reward_pools(
        &mut test_env.pic,
        &controller,
        &rewards_canister_id,
        &test_env.token_ledgers.values().cloned().collect(),
        100_000_000_000u64
    );

    // TRIGGER - synchronize_neurons
    test_env.pic.advance_time(Duration::from_millis(DAY_IN_MS));
    tick_n_blocks(&test_env.pic, 100);

    // TRIGGER - distribute_rewards
    test_env.pic.advance_time(Duration::from_millis(DAY_IN_MS * 6));
    tick_n_blocks(&test_env.pic, 100);
    test_env.pic.advance_time(Duration::from_secs(60 * 5));
    tick_n_blocks(&test_env.pic, 100);

    let neuron_sub_account = Account {
        owner: rewards_canister_id,
        subaccount: Some(neuron_id_1.clone().into()),
    };
    let neuron_icp_balance = balance_of(&test_env.pic, icp_ledger_id, neuron_sub_account);
    assert_eq!(neuron_icp_balance, expected_reward * 2);

    // ********************************
    // 4. There should be no active payment rounds
    // ********************************

    let active_payment_rounds = get_active_payment_rounds(
        &test_env.pic,
        controller,
        rewards_canister_id,
        &()
    );
    assert_eq!(active_payment_rounds.len(), 0);

    // ********************************
    // 4. neuron should have rewarded maturity
    // ********************************

    let single_neuron = get_neuron_by_id(
        &test_env.pic,
        controller,
        rewards_canister_id,
        &neuron_id_1
    ).unwrap();
    let rewarded_mat_icp = single_neuron.rewarded_maturity.get(&icp_token).unwrap();
    let rewarded_mat_ogy = single_neuron.rewarded_maturity.get(&ogy_token).unwrap();
    let rewarded_mat_gldgov = single_neuron.rewarded_maturity.get(&gldgov_token).unwrap();
    assert_eq!(rewarded_mat_icp, &200_000u64);
    assert_eq!(rewarded_mat_ogy, &200_000u64);
    assert_eq!(rewarded_mat_gldgov, &200_000u64);
}

// if there are no rewards in the reward pool then it should not distribute for that token. other's with rewards should carry on.
#[test]
fn test_distribute_rewards_with_no_rewards() {
    let mut test_env = default_test_setup();

    let icp_ledger_id = test_env.token_ledgers.get("icp_ledger_canister_id").unwrap().clone();
    let controller = test_env.controller;
    let rewards_canister_id = test_env.rewards_canister_id;
    let neuron_id_1 = test_env.neuron_data.get(&0usize).unwrap().clone().id.unwrap();

    let icp_token = TokenSymbol::parse("ICP").unwrap();
    let ogy_token = TokenSymbol::parse("OGY").unwrap();
    let gldgov_token = TokenSymbol::parse("GLDGov").unwrap();

    let reward_pool = Account {
        owner: rewards_canister_id,
        subaccount: Some(REWARD_POOL_SUB_ACCOUNT),
    };

    // ********************************
    // 1. Remove the entire balance of only the ICP reward pool
    // ********************************

    transfer(
        &mut test_env.pic,
        rewards_canister_id,
        icp_ledger_id,
        Some(REWARD_POOL_SUB_ACCOUNT),
        Account {
            owner: Principal::anonymous(),
            subaccount: None,
        },
        100_000_000_000u128 - 10_000u128
    ).unwrap();

    let icp_reward_pool_balance = balance_of(&test_env.pic, icp_ledger_id, reward_pool);
    assert_eq!(icp_reward_pool_balance, Nat::from(0u64));

    // ********************************
    // 2. Distribute rewards
    // ********************************

    test_env.simulate_neuron_voting(2);

    // TRIGGER - synchronize_neurons
    test_env.pic.advance_time(Duration::from_millis(DAY_IN_MS));
    tick_n_blocks(&test_env.pic, 100);

    // TRIGGER - distribute_rewards
    test_env.pic.advance_time(Duration::from_millis(DAY_IN_MS * 6));
    tick_n_blocks(&test_env.pic, 100);
    test_env.pic.advance_time(Duration::from_secs(60 * 5));
    tick_n_blocks(&test_env.pic, 100);

    // there should be no historic or active rounds for ICP because it didn't have any rewards to pay out
    let res = execute_update_multi_args::<(String, u16), Vec<(u16, PaymentRound)>>(
        &mut test_env.pic,
        Principal::anonymous(),
        rewards_canister_id,
        "get_historic_payment_round",
        ("ICP".to_string(), 1)
    );
    assert_eq!(res.len(), 0);
    let res = get_active_payment_rounds(
        &test_env.pic,
        Principal::anonymous(),
        rewards_canister_id,
        &()
    );
    assert_eq!(res.len(), 0);

    let single_neuron = get_neuron_by_id(
        &test_env.pic,
        Principal::anonymous(),
        rewards_canister_id,
        &neuron_id_1
    ).unwrap();
    let rewarded_mat_icp = single_neuron.rewarded_maturity.get(&icp_token);
    let rewarded_mat_ogy = single_neuron.rewarded_maturity.get(&ogy_token).unwrap();
    let rewarded_mat_gldgov = single_neuron.rewarded_maturity.get(&gldgov_token).unwrap();

    assert_eq!(rewarded_mat_icp, None);
    assert_eq!(rewarded_mat_ogy, &100_000u64);
    assert_eq!(rewarded_mat_gldgov, &100_000u64);

    // ********************************
    // 3. Distribute rewards - week 3 - ALL THREE now have rewards to distribute
    // ********************************
    setup_reward_pools(
        &mut test_env.pic,
        &controller,
        &rewards_canister_id,
        &test_env.token_ledgers.values().cloned().collect(),
        100_000_000_000u64
    );
    // increase maturity maturity
    test_env.simulate_neuron_voting(3);

    // TRIGGER - synchronize_neurons
    test_env.pic.advance_time(Duration::from_millis(DAY_IN_MS));
    tick_n_blocks(&test_env.pic, 100);

    // TRIGGER - distribute_rewards
    test_env.pic.advance_time(Duration::from_millis(DAY_IN_MS * 6));
    tick_n_blocks(&test_env.pic, 100);
    test_env.pic.advance_time(Duration::from_secs(60 * 5));
    tick_n_blocks(&test_env.pic, 100);

    // test historic rounds - note, payment round id's always go up by 1 if any rewards from any token are distributed so we get ("ICP".to_string(), 2)
    let res = execute_update_multi_args::<(String, u16), Vec<(u16, PaymentRound)>>(
        &mut test_env.pic,
        Principal::anonymous(),
        rewards_canister_id,
        "get_historic_payment_round",
        ("ICP".to_string(), 2)
    );
    assert_eq!(res.len(), 1);

    let single_neuron = get_neuron_by_id(
        &test_env.pic,
        Principal::anonymous(),
        rewards_canister_id,
        &neuron_id_1
    ).unwrap();
    let rewarded_mat_icp = single_neuron.rewarded_maturity.get(&icp_token).unwrap();
    let rewarded_mat_ogy = single_neuron.rewarded_maturity.get(&ogy_token).unwrap();
    let rewarded_mat_gldgov = single_neuron.rewarded_maturity.get(&gldgov_token).unwrap();
    assert_eq!(rewarded_mat_icp, &200_000u64);
    assert_eq!(rewarded_mat_ogy, &200_000u64);
    assert_eq!(rewarded_mat_gldgov, &200_000u64);
}

// if 1 reward pool doesn't have enough rewards it should be skipped
#[test]
fn test_distribute_rewards_with_not_enough_rewards() {
    let mut test_env = default_test_setup();

    let icp_ledger_id = test_env.token_ledgers.get("icp_ledger_canister_id").unwrap().clone();
    let ogy_ledger_id = test_env.token_ledgers.get("ogy_ledger_canister_id").unwrap().clone();
    let gldgov_ledger_id = test_env.token_ledgers.get("gldgov_ledger_canister_id").unwrap().clone();
    let rewards_canister_id = test_env.rewards_canister_id;

    // ********************************
    // 1. Give ICP reward pool balance less than the total in fees
    // ********************************
    let reward_pool = Account {
        owner: rewards_canister_id,
        subaccount: Some(REWARD_POOL_SUB_ACCOUNT),
    };
    // calculate the minimum balance
    let minimum_reward_pool_required = 10_000u64 * (test_env.neuron_data.len() as u64) + 10_000u64;
    let bad_starting_reward_amount = minimum_reward_pool_required - 10_000;
    // transfer from reward pool to some random id
    transfer(
        &mut test_env.pic,
        rewards_canister_id,
        icp_ledger_id,
        Some(REWARD_POOL_SUB_ACCOUNT),
        Account {
            owner: Principal::anonymous(),
            subaccount: None,
        },
        100_000_000_000u128 - 10_000u128 - (bad_starting_reward_amount as u128)
    ).unwrap();

    let icp_reward_pool_balance = balance_of(&test_env.pic, icp_ledger_id, reward_pool);
    assert_eq!(icp_reward_pool_balance, Nat::from(bad_starting_reward_amount));

    let ogy_reward_pool_balance = balance_of(&test_env.pic, ogy_ledger_id, reward_pool);
    assert_eq!(ogy_reward_pool_balance, Nat::from(100_000_000_000u64));

    let gldgov_reward_pool_balance = balance_of(&test_env.pic, gldgov_ledger_id, reward_pool);
    assert_eq!(gldgov_reward_pool_balance, Nat::from(100_000_000_000u64));

    // ********************************
    // 2. Distribute rewards
    // ********************************

    // increase maturity maturity
    test_env.simulate_neuron_voting(2);

    // TRIGGER - synchronize_neurons
    test_env.pic.advance_time(Duration::from_millis(DAY_IN_MS));
    tick_n_blocks(&test_env.pic, 100);

    // TRIGGER - distribute_rewards
    test_env.pic.advance_time(Duration::from_millis(DAY_IN_MS * 6));
    tick_n_blocks(&test_env.pic, 100);
    test_env.pic.advance_time(Duration::from_secs(60 * 5));
    tick_n_blocks(&test_env.pic, 100);

    // there should be no historic payment round for ICP
    let res = execute_update_multi_args::<(String, u16), Vec<(u16, PaymentRound)>>(
        &mut test_env.pic,
        Principal::anonymous(),
        rewards_canister_id,
        "get_historic_payment_round",
        ("ICP".to_string(), 1)
    );
    assert_eq!(res.len(), 0);
    // there should be no active round for ICP
    let p = get_active_payment_rounds(
        &test_env.pic,
        Principal::anonymous(),
        rewards_canister_id,
        &()
    );
    assert_eq!(p.len(), 0);

    // the others should have historic rounds
    let res = execute_update_multi_args::<(String, u16), Vec<(u16, PaymentRound)>>(
        &mut test_env.pic,
        Principal::anonymous(),
        rewards_canister_id,
        "get_historic_payment_round",
        ("OGY".to_string(), 1)
    );
    assert_eq!(res.len(), 1);
    let res = execute_update_multi_args::<(String, u16), Vec<(u16, PaymentRound)>>(
        &mut test_env.pic,
        Principal::anonymous(),
        rewards_canister_id,
        "get_historic_payment_round",
        ("GLDGov".to_string(), 1)
    );
    assert_eq!(res.len(), 1);
}