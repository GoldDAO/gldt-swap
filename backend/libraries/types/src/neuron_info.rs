use std::borrow::Cow;

use candid::{ CandidType, Decode, Encode };
use ic_stable_structures::{ storable::Bound, Storable };
use serde::{ Deserialize, Serialize };

const MAX_VALUE_SIZE: u32 = 100;

/// The maturity information about a neuron
#[derive(Serialize, Clone, Deserialize, CandidType, Copy, Debug, PartialEq, Eq)]
pub struct NeuronInfo {
    pub last_synced_maturity: u64,
    pub accumulated_maturity: u64,
}

impl Storable for NeuronInfo {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }
    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(&bytes, Self).unwrap()
    }
    const BOUND: Bound = Bound::Bounded { max_size: MAX_VALUE_SIZE, is_fixed_size: false };
}
