use ic_cdk::export_candid;
use management_api_canister::init::InitArgs;

mod guards;
pub mod lifecycle;
pub mod queries;
pub mod state;
pub mod memory;
pub mod updates;

use updates::*;
use queries::*;

export_candid!();
