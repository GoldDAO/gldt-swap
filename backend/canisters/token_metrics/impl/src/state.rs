use candid::{ CandidType, Principal };
use types::TimestampMillis;
use utils::{
    consts::{ ICP_LEDGER_CANISTER_ID, NNS_GOVERNANCE_CANISTER_ID, SNS_GOVERNANCE_CANISTER_ID },
    env::{ CanisterEnv, Environment },
    memory::MemorySize,
};
use canister_state_macros::canister_state;
use serde::{ Serialize, Deserialize };

canister_state!(RuntimeState);

#[derive(Serialize, Deserialize)]
pub struct RuntimeState {
    /// Runtime environment
    pub env: CanisterEnv,
    /// Runtime data
    pub data: Data,
}

impl RuntimeState {
    pub fn new(env: CanisterEnv, data: Data) -> Self {
        Self { env, data }
    }

    pub fn metrics(&self) -> Metrics {
        Metrics {
            canister_info: CanisterInfo {
                now: self.env.now(),
                test_mode: self.env.is_test_mode(),
                memory_used: MemorySize::used(),
                cycles_balance_in_tc: self.env.cycles_balance_in_tc(),
            },
        }
    }
}

#[derive(CandidType, Serialize)]
pub struct Metrics {
    pub canister_info: CanisterInfo,
}
#[derive(CandidType, Deserialize, Serialize)]
pub struct CanisterInfo {
    pub now: TimestampMillis,
    pub test_mode: bool,
    pub memory_used: MemorySize,
    pub cycles_balance_in_tc: f64,
}
#[derive(Serialize, Deserialize)]
pub struct Data {
    pub gold_price: f64,
    pub gold_nft_canisters: Vec<(Principal, u128)>,
    pub total_gold_grams: u128,
}

impl Data {
    pub fn new(gold_nft_canisters: Vec<(Principal, u128)>) -> Self {
        Self {
            gold_price: 0.0,
            gold_nft_canisters: gold_nft_canisters,
            total_gold_grams: 0,
        }
    }
}