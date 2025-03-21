use crate::guards::caller_is_governance_principal;
use canister_tracing_macros::trace;
use ic_cdk::{query, update};

#[query(guard = "caller_is_governance_principal", hidden = true)]
#[trace]
async fn process_burn_validate() -> Result<String, String> {
    Ok("No arguments to validate".to_string())
}

#[update(guard = "caller_is_governance_principal", hidden = true)]
pub fn process_burn() {
    crate::jobs::burn_tokens::run()
}
