use candid::CandidType;
use serde::{Deserialize, Serialize};
use types::{BuildVersion, CanisterId};

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct InitArgs {
    pub test_mode: bool,
    pub version: BuildVersion,
    pub commit_hash: String,
    pub sns_governance_canister_id: CanisterId,
    pub super_stats_canister_id: CanisterId,
    pub ogy_new_ledger_canister_id: CanisterId,
    pub gldt_ledger_canister_id: CanisterId,
    pub sns_rewards_canister_id: CanisterId,
    pub treasury_account: String,
    // b3j22-o6yuq-jdcbo-kk7js-jm24t-q6hkj-hf5fd-bhizo-2erxi-6gk5s-qae.0000000000000000000000000000000000000000000000000000000000000000
    // 2gytz-5mjny-5qfcl-vjsle-654l2-ixgif-3vfqj-nryxk-uzgfx-5df5u-sqe.0000000000000000000000000000000000000000000000000000000000000000
    pub foundation_accounts: Vec<String>,
}
