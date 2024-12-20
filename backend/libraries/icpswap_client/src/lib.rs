use candid::{CandidType, Nat};
use ic_cdk::api::call::{CallResult, RejectionCode};
use ic_ledger_types::Subaccount;
use icpswap_swap_pool_canister::{ICPSwapError, ICPSwapResult};
use icrc_ledger_types::icrc1::account::Account;
use serde::{Deserialize, Serialize};
use types::{CanisterId, TokenInfo};

// NOTE: we use one ICPSwapClient to swap concrete token pair
#[derive(CandidType, Serialize, Deserialize, Clone)]
pub struct ICPSwapClient {
    client_id: u128,
    this_canister_id: CanisterId,
    swap_canister_id: CanisterId,
    token0: TokenInfo,
    token1: TokenInfo,
    zero_for_one: bool,
}

impl ICPSwapClient {
    pub fn new(
        client_id: u128,
        this_canister_id: CanisterId,
        swap_canister_id: CanisterId,
        token0: TokenInfo,
        token1: TokenInfo,
        zero_for_one: bool,
    ) -> Self {
        ICPSwapClient {
            client_id,
            this_canister_id,
            swap_canister_id,
            token0,
            token1,
            zero_for_one,
        }
    }

    pub fn deposit_account_internal(&self) -> Account {
        Account {
            owner: self.swap_canister_id,
            subaccount: Some(Subaccount::from(self.this_canister_id).0),
        }
    }

    pub async fn get_quote(
        &self,
        amount: u128,
        min_amount_out: u128,
    ) -> CallResult<Result<u128, String>> {
        let args = icpswap_swap_pool_canister::swap::Args {
            operator: self.this_canister_id,
            amount_in: amount.to_string(),
            zero_for_one: self.zero_for_one,
            amount_out_minimum: min_amount_out.to_string(),
        };
        match icpswap_swap_pool_canister_c2c_client::quote(self.swap_canister_id, &args).await? {
            ICPSwapResult::Ok(amount_out) => Ok(Ok(nat_to_u128(amount_out))),
            ICPSwapResult::Err(error) => Ok(Err(format!("{error:?}"))),
        }
    }

    // NOTE: ICPSwap API - https://dashboard.internetcomputer.org/canister/7eikv-2iaaa-aaaag-qdgwa-cai
    pub async fn deposit(&self, amount: u128) -> CallResult<u128> {
        let token = self.input_token();
        let args = icpswap_swap_pool_canister::deposit::Args {
            token: token.ledger_id.to_string(),
            amount: amount.into(),
            fee: token.fee.into(),
        };
        match icpswap_swap_pool_canister_c2c_client::deposit(self.swap_canister_id, &args).await? {
            ICPSwapResult::Ok(amount_deposited) => Ok(nat_to_u128(amount_deposited)),
            ICPSwapResult::Err(error) => Err(convert_error(error)),
        }
    }

    pub async fn swap(
        &self,
        amount: u128,
        min_amount_out: u128,
    ) -> CallResult<Result<u128, String>> {
        let args = icpswap_swap_pool_canister::swap::Args {
            operator: self.this_canister_id,
            amount_in: amount.to_string(),
            zero_for_one: self.zero_for_one,
            amount_out_minimum: min_amount_out.to_string(),
        };
        match icpswap_swap_pool_canister_c2c_client::swap(self.swap_canister_id, &args).await? {
            ICPSwapResult::Ok(amount_out) => Ok(Ok(nat_to_u128(amount_out))),
            ICPSwapResult::Err(error) => Ok(Err(format!("{error:?}"))),
        }
    }

    pub async fn withdraw(&self, successful_swap: bool, amount: u128) -> CallResult<u128> {
        let token = if successful_swap {
            self.output_token()
        } else {
            self.input_token()
        };
        let args = icpswap_swap_pool_canister::withdraw::Args {
            token: token.ledger_id.to_string(),
            amount: amount.into(),
            fee: token.fee.into(),
        };
        match icpswap_swap_pool_canister_c2c_client::withdraw(self.swap_canister_id, &args).await? {
            ICPSwapResult::Ok(amount_out) => Ok(nat_to_u128(amount_out)),
            ICPSwapResult::Err(error) => Err(convert_error(error)),
        }
    }

    pub fn client_id(&self) -> u128 {
        self.client_id
    }

    pub fn input_token(&self) -> TokenInfo {
        if self.zero_for_one {
            self.token0
        } else {
            self.token1
        }
    }

    pub fn output_token(&self) -> TokenInfo {
        if self.zero_for_one {
            self.token1
        } else {
            self.token0
        }
    }

    pub fn this_canister_id(&self) -> CanisterId {
        self.this_canister_id
    }

    pub fn swap_canister_id(&self) -> CanisterId {
        self.swap_canister_id
    }

    pub fn set_swap_canister_id(&mut self, swap_canister_id: CanisterId) {
        self.swap_canister_id = swap_canister_id;
    }

    pub fn zero_for_one(&self) -> bool {
        self.zero_for_one
    }
}

fn nat_to_u128(value: Nat) -> u128 {
    value.0.try_into().unwrap()
}

fn convert_error(error: ICPSwapError) -> (RejectionCode, String) {
    (RejectionCode::Unknown, format!("{error:?}"))
}
