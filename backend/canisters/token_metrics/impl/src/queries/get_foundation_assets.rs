use crate::state::read_state;
use ic_cdk_macros::query;
pub use token_metrics_api::queries::get_foundation_assets::Response as GetFoundationAssetsResponse;

#[query]
fn get_foundation_assets() -> GetFoundationAssetsResponse {
    read_state(|state| state.data.foundation_accounts_data.clone())
}
