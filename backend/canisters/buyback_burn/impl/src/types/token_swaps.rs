use crate::memory::get_swap_history_memory;
use crate::memory::VM;
use crate::types::*;
use buyback_burn_api::get_active_swaps::Response;
use candid::CandidType;
use ic_stable_structures::StableBTreeMap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::error;
use types::TimestampMillis;

#[derive(Serialize, Deserialize)]
pub struct TokenSwaps {
    swaps: HashMap<u128, TokenSwap>,
    #[serde(skip, default = "init_map")]
    history: StableBTreeMap<u128, TokenSwap, VM>,
}

fn init_map() -> StableBTreeMap<u128, TokenSwap, VM> {
    let memory = get_swap_history_memory();
    StableBTreeMap::init(memory)
}

impl Default for TokenSwaps {
    fn default() -> Self {
        Self {
            swaps: HashMap::new(),
            history: init_map(),
        }
    }
}

impl TokenSwaps {
    pub fn push_new(&mut self, swap_config: SwapConfig, now: TimestampMillis) -> TokenSwap {
        let id = self.get_next_id();
        let token_swap = TokenSwap::new(id, swap_config.swap_client_id, now);
        self.upsert(token_swap.clone());
        token_swap
    }

    pub fn upsert(&mut self, swap: TokenSwap) {
        self.swaps.insert(swap.swap_id, swap);
    }

    pub fn get(&self, swap_id: u128) -> Option<&TokenSwap> {
        self.swaps.get(&swap_id)
    }

    pub fn iter(&self) -> impl Iterator<Item = &TokenSwap> {
        self.swaps.values()
    }

    pub fn get_next_id(&self) -> u128 {
        let swaps_len: u128 = self.swaps.len().try_into().unwrap();
        let history_len: u128 = self.history.len().into();
        swaps_len + history_len + 1
    }

    pub fn get_swap_info(&self, swap_id: u128) -> Option<TokenSwap> {
        let swap_info_incomplete = self.swaps.get(&swap_id).cloned();
        let swap_info_completed = self.history.get(&swap_id);
        swap_info_incomplete.or(swap_info_completed)
    }

    pub fn archive_swap(&mut self, swap_id: u128) -> Result<(), String> {
        let swap_info = self.swaps.get(&swap_id);
        match swap_info {
            Some(swap) => {
                let mut modified_swap = swap.clone();
                modified_swap.is_archived = true;
                self.history.insert(swap_id, modified_swap.clone());
                self.swaps.remove(&swap_id);
                Ok(())
            }
            None => {
                error!("Failed to archive {swap_id}. Swap not found");
                Err(format!("Failed to archive {}. Swap not found", swap_id))
            }
        }
    }

    pub fn get_active_swaps(&self) -> Response {
        self.swaps.clone()
    }

    pub fn get_metrics(&self) -> TokenSwapsMetrics {
        TokenSwapsMetrics {
            active_swaps: self.swaps.clone(),
            active_swaps_len: self.swaps.len() as u64,
            history_len: self.history.len(),
        }
    }
}

#[derive(CandidType, Serialize)]
pub struct TokenSwapsMetrics {
    active_swaps: HashMap<u128, TokenSwap>,
    active_swaps_len: u64,
    history_len: u64,
}
