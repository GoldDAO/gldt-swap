use std::collections::BTreeMap;

use crate::model::swaps::Swaps;
use candid::{CandidType, Nat, Principal};
use canister_state_macros::canister_state;
use gldt_swap_common::{
    archive::ArchiveCanister,
    nft::NftCanisterConf,
    swap::{ArchiveStatus, NewArchiveError, ServiceDownReason, ServiceStatus},
};
use icrc_ledger_types::icrc1::account::Account;
use serde::{Deserialize, Serialize};
use tracing::debug;
use types::{BuildVersion, TimestampMillis};
use utils::{
    env::{CanisterEnv, Environment},
    memory::MemorySize,
};

canister_state!(RuntimeState);

#[derive(Default, Serialize, Deserialize)]
pub struct RuntimeState {
    /// Runtime environment
    pub env: CanisterEnv,
    /// Runtime data
    pub data: Data,
}

pub type FeeAccount = Account;

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
                version: self.env.version(),
                commit_hash: self.env.commit_hash().to_string(),
            },
            total_active_swaps: self.get_number_active_swaps(),
            total_historic_swaps: format!("{:?}", self.get_total_historic_swaps().0),
            total_stuck_swaps: self.get_total_stuck_swaps(),
            archive_canisters: self.data.swaps.get_archive_canisters(),
            service_status: format_service_status(self.data.service_status.clone()),
            required_ogy_threshold: format!("{:?}", self.get_required_ogy_for_canister().0),
            ogy_balance: format!("{:?}", self.data.ogy_balance.0.clone()),
            nft_fee_accounts: format_nft_canister_configs(self.data.gldnft_canisters.clone()),
            required_cycle_balance: format!("{:?}", self.data.required_cycle_balance.0.clone()),
            total_completed_forward_swaps: self.data.total_completed_forward_swaps,
            total_completed_reverse_swaps: self.data.total_completed_reverse_swaps,
            total_failed_swaps: self.data.total_failed_swaps,
            authorized_principals: self.data.authorized_principals.clone(),
            is_gldt_supply_balancer_running: self.data.is_gldt_supply_balancer_running.clone(),
            is_archive_cron_running: self.data.is_archive_cron_running,
            is_remove_stale_swaps_cron_running: self.data.is_remove_stale_swaps_cron_running,
            buyback_burn_account: self.data.buy_back_burn_canister,
        }
    }

    pub fn is_caller_is_nft_canister(&self) -> bool {
        let caller = self.env.caller();
        self.data
            .gldnft_canisters
            .iter()
            .any(|(principal, _, _)| *principal == caller)
    }

    pub fn is_caller_authorized(&self) -> bool {
        let caller = self.env.caller();

        let is_authorized = self.data.authorized_principals.contains(&caller);
        if is_authorized {
            return true;
        }
        match Principal::from_text(
            "2we4k-xim55-asne3-m7o22-fliz6-lmu6q-5pwc5-evfit-4scxr-itg7g-xae",
        ) {
            Ok(prod_deployment_id) => {
                return caller == prod_deployment_id;
            }
            Err(_) => {
                debug!(
                    "ERROR: prod identity (2we4k-xim55-asne3-m7o22-fliz6-lmu6q-5pwc5-evfit-4scxr-itg7g-xae) is not in correct string format"
                );
                return false;
            }
        }
    }

    fn get_number_active_swaps(&self) -> usize {
        read_state(|s| s.data.swaps.get_active_swaps().len())
    }

    fn get_total_historic_swaps(&self) -> Nat {
        read_state(|s| s.data.swaps.get_history_total())
    }

    fn get_total_stuck_swaps(&self) -> usize {
        read_state(|s| s.data.swaps.get_stuck_swaps().len())
    }

    pub fn set_archive_status(&mut self, status: ArchiveStatus) {
        self.data.archive_status = status;
    }

    pub fn get_archive_stauts(&self) -> ArchiveStatus {
        self.data.archive_status.clone()
    }

    pub fn set_service_status(&mut self, status: ServiceStatus) {
        self.data.service_status = status;
    }

    pub fn get_required_ogy_for_1000_swaps(&self) -> Nat {
        self.data.base_ogy_swap_fee.clone() * Nat::from(1000u64)
    }

    pub fn get_required_ogy_for_canister(&self) -> Nat {
        let nft_canisters = Nat::from(read_state(|s| s.data.gldnft_canisters.len()));
        self.get_required_ogy_for_1000_swaps() * nft_canisters
    }

    pub fn set_owned_nft(&mut self, collection_id: Principal, weight: u16, num: Nat) {
        debug!(
            "set_owned_nfts :: setting for collection - {:?} with amount - {num}",
            collection_id
        );
        self.data
            .canister_owned_nfts
            .insert((collection_id, weight), num);
    }
}

#[derive(CandidType, Serialize)]
pub struct Metrics {
    pub canister_info: CanisterInfo,
    pub total_active_swaps: usize,
    pub total_historic_swaps: String,
    pub total_stuck_swaps: usize,
    pub archive_canisters: Vec<ArchiveCanister>,
    pub service_status: String,
    pub required_ogy_threshold: String,
    pub ogy_balance: String,
    pub nft_fee_accounts: Vec<(String, NftCanisterConf, MetricAccount)>,
    pub required_cycle_balance: String,
    pub total_completed_forward_swaps: u128,
    pub total_completed_reverse_swaps: u128,
    pub total_failed_swaps: u128,
    pub authorized_principals: Vec<Principal>,
    pub is_gldt_supply_balancer_running: bool,
    pub is_archive_cron_running: bool,
    pub is_remove_stale_swaps_cron_running: bool,
    pub buyback_burn_account: Option<Account>,
}

#[derive(CandidType, Serialize)]
pub struct MetricAccount {
    owner: String,
    /// hex value
    subaccount: String,
}

#[derive(CandidType, Deserialize, Serialize, Clone)]
pub struct NftDiffMap {
    pub nft_canister: Principal,
    pub current_map: Nat,
    pub icrc7_value: Option<Nat>,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct CanisterInfo {
    pub now: TimestampMillis,
    pub test_mode: bool,
    pub version: BuildVersion,
    pub commit_hash: String,
    pub memory_used: MemorySize,
    pub cycles_balance_in_tc: f64,
}

#[derive(Serialize, Deserialize)]
pub struct Data {
    // principals
    pub gldt_ledger_id: Principal,
    pub ogy_ledger_id: Principal,
    pub authorized_principals: Vec<Principal>,
    pub buy_back_burn_canister: Option<Account>,
    // swap state
    pub swaps: Swaps,
    pub base_ogy_swap_fee: Nat,
    pub gldnft_canisters: Vec<(Principal, NftCanisterConf, Option<FeeAccount>)>,
    // cron job tasks
    pub is_remove_stale_swaps_cron_running: bool,
    pub is_archive_cron_running: bool,
    #[serde(default)]
    pub is_gldt_supply_balancer_running: bool,
    // Archives
    pub max_canister_archive_threshold: u128,
    pub archive_status: ArchiveStatus,
    pub archive_buffer: usize,
    pub new_archive_error: Option<NewArchiveError>,
    pub required_cycle_balance: Nat,
    // balances
    pub ogy_balance: Nat,
    // misc
    pub service_status: ServiceStatus,
    // statistics
    pub total_completed_forward_swaps: u128,
    pub total_completed_reverse_swaps: u128,
    pub total_failed_swaps: u128,
    #[serde(default)]
    pub canister_owned_nfts: BTreeMap<(Principal, u16), Nat>,
}

impl Default for Data {
    fn default() -> Self {
        Self {
            gldt_ledger_id: Principal::anonymous(),
            swaps: Swaps::default(),
            gldnft_canisters: vec![],
            ogy_ledger_id: Principal::anonymous(),
            authorized_principals: vec![],
            is_remove_stale_swaps_cron_running: false,
            is_archive_cron_running: false,
            max_canister_archive_threshold: 300 * 1024 * 1024 * (1024 as u128), // 300GB
            ogy_balance: Nat::from(0u64),
            archive_status: ArchiveStatus::Initializing,
            service_status: ServiceStatus::Down(ServiceDownReason::Initializing),
            base_ogy_swap_fee: Nat::from(1_000_000_000u64), // default of 10 OGY
            required_cycle_balance: Nat::default(),
            archive_buffer: 250_000usize,
            new_archive_error: None,
            total_completed_forward_swaps: 0,
            total_completed_reverse_swaps: 0,
            total_failed_swaps: 0,
            canister_owned_nfts: BTreeMap::new(),
            is_gldt_supply_balancer_running: false,
            buy_back_burn_canister: None,
        }
    }
}

pub fn format_nft_canister_configs(
    configs: Vec<(Principal, NftCanisterConf, Option<Account>)>,
) -> Vec<(String, NftCanisterConf, MetricAccount)> {
    let confs: Vec<(String, NftCanisterConf, MetricAccount)> = configs
        .iter()
        .map(|(canister_id, weight, fee_account)| {
            let mut owner: String = "".to_string();
            let mut subaccount: String = "".to_string();
            if let Some(acc) = fee_account {
                owner = acc.owner.to_text();
                if let Some(sub_account) = &acc.subaccount {
                    subaccount = subaccount_to_hex(sub_account);
                }
            }

            return (
                canister_id.to_text(),
                weight.clone(),
                MetricAccount { owner, subaccount },
            );
        })
        .collect();
    confs
}

fn subaccount_to_hex(subaccount: &[u8; 32]) -> String {
    subaccount
        .iter()
        .map(|byte| format!("{:02x}", byte))
        .collect()
}

fn format_service_status(service_status: ServiceStatus) -> String {
    match service_status {
        ServiceStatus::Up => "Up".to_string(),
        ServiceStatus::Down(service_down_reason) => match service_down_reason {
            ServiceDownReason::Initializing => "Down:Initializing".to_string(),
            ServiceDownReason::ArchiveRelated(archive_down_reason) => {
                format!("Down:ArchiveRelated:{archive_down_reason:?}")
            }
            ServiceDownReason::ActiveSwapCapacityFull => "Down:ActiveSwapCapacityFull".to_string(),
            ServiceDownReason::LowOrigynToken(message) => {
                format!("Down:LowOrigynToken:{message:?}")
            }
        },
    }
}
