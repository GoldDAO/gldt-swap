// This is an experimental feature to generate Rust binding from Candid.
// You may want to manually adjust some of the types.
#![allow(dead_code, unused_imports)]
use candid::{self, CandidType, Decode, Deserialize, Encode, Principal};

#[derive(CandidType, Deserialize, Clone)]
pub struct NeuronBasketConstructionParameters {
    pub dissolve_delay_interval_seconds: u64,
    pub count: u64,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct LinearScalingCoefficient {
    pub slope_numerator: Option<u64>,
    pub intercept_icp_e8s: Option<u64>,
    pub from_direct_participation_icp_e8s: Option<u64>,
    pub slope_denominator: Option<u64>,
    pub to_direct_participation_icp_e8s: Option<u64>,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct IdealMatchedParticipationFunction {
    pub serialized_representation: Option<String>,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct NeuronsFundParticipationConstraints {
    pub coefficient_intervals: Vec<LinearScalingCoefficient>,
    pub max_neurons_fund_participation_icp_e8s: Option<u64>,
    pub min_direct_participation_threshold_icp_e8s: Option<u64>,
    pub ideal_matched_participation_function: Option<IdealMatchedParticipationFunction>,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct Countries {
    pub iso_codes: Vec<String>,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct Init {
    pub nns_proposal_id: Option<u64>,
    pub sns_root_canister_id: String,
    pub neurons_fund_participation: Option<bool>,
    pub min_participant_icp_e8s: Option<u64>,
    pub neuron_basket_construction_parameters: Option<NeuronBasketConstructionParameters>,
    pub fallback_controller_principal_ids: Vec<String>,
    pub max_icp_e8s: Option<u64>,
    pub neuron_minimum_stake_e8s: Option<u64>,
    pub confirmation_text: Option<String>,
    pub swap_start_timestamp_seconds: Option<u64>,
    pub swap_due_timestamp_seconds: Option<u64>,
    pub min_participants: Option<u32>,
    pub sns_token_e8s: Option<u64>,
    pub nns_governance_canister_id: String,
    pub transaction_fee_e8s: Option<u64>,
    pub icp_ledger_canister_id: String,
    pub sns_ledger_canister_id: String,
    pub neurons_fund_participation_constraints: Option<NeuronsFundParticipationConstraints>,
    pub should_auto_finalize: Option<bool>,
    pub max_participant_icp_e8s: Option<u64>,
    pub sns_governance_canister_id: String,
    pub min_direct_participation_icp_e8s: Option<u64>,
    pub restricted_countries: Option<Countries>,
    pub min_icp_e8s: Option<u64>,
    pub max_direct_participation_icp_e8s: Option<u64>,
}

#[derive(CandidType, Deserialize)]
pub struct ErrorRefundIcpRequest {
    pub source_principal_id: Option<Principal>,
}

#[derive(CandidType, Deserialize)]
pub struct Ok {
    pub block_height: Option<u64>,
}

#[derive(CandidType, Deserialize)]
pub struct Err {
    pub description: Option<String>,
    pub error_type: Option<i32>,
}

#[derive(CandidType, Deserialize)]
pub enum Result_ {
    Ok(Ok),
    Err(Err),
}

#[derive(CandidType, Deserialize)]
pub struct ErrorRefundIcpResponse {
    pub result: Option<Result_>,
}

#[derive(CandidType, Deserialize)]
pub struct FinalizeSwapArg {}

#[derive(CandidType, Deserialize)]
pub struct CanisterCallError {
    pub code: Option<i32>,
    pub description: String,
}

#[derive(CandidType, Deserialize)]
pub struct FailedUpdate {
    pub err: Option<CanisterCallError>,
    pub dapp_canister_id: Option<Principal>,
}

#[derive(CandidType, Deserialize)]
pub struct SetDappControllersResponse {
    pub failed_updates: Vec<FailedUpdate>,
}

#[derive(CandidType, Deserialize)]
pub enum Possibility {
    Ok(SetDappControllersResponse),
    Err(CanisterCallError),
}

#[derive(CandidType, Deserialize)]
pub struct SetDappControllersCallResult {
    pub possibility: Option<Possibility>,
}

#[derive(CandidType, Deserialize)]
pub struct SweepResult {
    pub failure: u32,
    pub skipped: u32,
    pub invalid: u32,
    pub success: u32,
    pub global_failures: u32,
}

#[derive(CandidType, Deserialize)]
pub struct GovernanceError {
    pub error_message: String,
    pub error_type: i32,
}

#[derive(CandidType, Deserialize)]
pub struct Response {
    pub governance_error: Option<GovernanceError>,
}

#[derive(CandidType, Deserialize)]
pub enum Possibility1 {
    Ok(Response),
    Err(CanisterCallError),
}

#[derive(CandidType, Deserialize)]
pub struct SettleCommunityFundParticipationResult {
    pub possibility: Option<Possibility1>,
}

#[derive(CandidType, Deserialize)]
pub struct Ok1 {
    pub neurons_fund_participation_icp_e8s: Option<u64>,
    pub neurons_fund_neurons_count: Option<u64>,
}

#[derive(CandidType, Deserialize)]
pub struct Error {
    pub message: Option<String>,
}

#[derive(CandidType, Deserialize)]
pub enum Possibility2 {
    Ok(Ok1),
    Err(Error),
}

#[derive(CandidType, Deserialize)]
pub struct SettleNeuronsFundParticipationResult {
    pub possibility: Option<Possibility2>,
}

#[derive(CandidType, Deserialize)]
pub enum Possibility3 {
    Ok {},
    Err(CanisterCallError),
}

#[derive(CandidType, Deserialize)]
pub struct SetModeCallResult {
    pub possibility: Option<Possibility3>,
}

#[derive(CandidType, Deserialize)]
pub struct FinalizeSwapResponse {
    pub set_dapp_controllers_call_result: Option<SetDappControllersCallResult>,
    pub create_sns_neuron_recipes_result: Option<SweepResult>,
    pub settle_community_fund_participation_result: Option<SettleCommunityFundParticipationResult>,
    pub error_message: Option<String>,
    pub settle_neurons_fund_participation_result: Option<SettleNeuronsFundParticipationResult>,
    pub set_mode_call_result: Option<SetModeCallResult>,
    pub sweep_icp_result: Option<SweepResult>,
    pub claim_neuron_result: Option<SweepResult>,
    pub sweep_sns_result: Option<SweepResult>,
}

#[derive(CandidType, Deserialize)]
pub struct GetAutoFinalizationStatusArg {}

#[derive(CandidType, Deserialize)]
pub struct GetAutoFinalizationStatusResponse {
    pub auto_finalize_swap_response: Option<FinalizeSwapResponse>,
    pub has_auto_finalize_been_attempted: Option<bool>,
    pub is_auto_finalize_enabled: Option<bool>,
}

#[derive(CandidType, Deserialize)]
pub struct GetBuyerStateRequest {
    pub principal_id: Option<Principal>,
}

#[derive(CandidType, Deserialize)]
pub struct TransferableAmount {
    pub transfer_fee_paid_e8s: Option<u64>,
    pub transfer_start_timestamp_seconds: u64,
    pub amount_e8s: u64,
    pub amount_transferred_e8s: Option<u64>,
    pub transfer_success_timestamp_seconds: u64,
}

#[derive(CandidType, Deserialize)]
pub struct BuyerState {
    pub icp: Option<TransferableAmount>,
    pub has_created_neuron_recipes: Option<bool>,
}

#[derive(CandidType, Deserialize)]
pub struct GetBuyerStateResponse {
    pub buyer_state: Option<BuyerState>,
}

#[derive(CandidType, Deserialize)]
pub struct GetBuyersTotalArg {}

#[derive(CandidType, Deserialize)]
pub struct GetBuyersTotalResponse {
    pub buyers_total: u64,
}

#[derive(CandidType, Deserialize)]
pub struct GetCanisterStatusArg {}

#[derive(CandidType, Deserialize)]
pub enum CanisterStatusType {
    #[serde(rename = "stopped")]
    Stopped,
    #[serde(rename = "stopping")]
    Stopping,
    #[serde(rename = "running")]
    Running,
}

#[derive(CandidType, Deserialize)]
pub struct DefiniteCanisterSettingsArgs {
    pub freezing_threshold: candid::Nat,
    pub wasm_memory_threshold: Option<candid::Nat>,
    pub controllers: Vec<Principal>,
    pub wasm_memory_limit: Option<candid::Nat>,
    pub memory_allocation: candid::Nat,
    pub compute_allocation: candid::Nat,
}

#[derive(CandidType, Deserialize)]
pub struct QueryStats {
    pub response_payload_bytes_total: Option<candid::Nat>,
    pub num_instructions_total: Option<candid::Nat>,
    pub num_calls_total: Option<candid::Nat>,
    pub request_payload_bytes_total: Option<candid::Nat>,
}

#[derive(CandidType, Deserialize)]
pub struct CanisterStatusResultV2 {
    pub status: CanisterStatusType,
    pub memory_size: candid::Nat,
    pub cycles: candid::Nat,
    pub settings: DefiniteCanisterSettingsArgs,
    pub query_stats: Option<QueryStats>,
    pub idle_cycles_burned_per_day: candid::Nat,
    pub module_hash: Option<serde_bytes::ByteBuf>,
}

#[derive(CandidType, Deserialize)]
pub struct GetDerivedStateArg {}

#[derive(CandidType, Deserialize)]
pub struct GetDerivedStateResponse {
    pub sns_tokens_per_icp: Option<f64>,
    pub buyer_total_icp_e8s: Option<u64>,
    pub cf_participant_count: Option<u64>,
    pub neurons_fund_participation_icp_e8s: Option<u64>,
    pub direct_participation_icp_e8s: Option<u64>,
    pub direct_participant_count: Option<u64>,
    pub cf_neuron_count: Option<u64>,
}

#[derive(CandidType, Deserialize)]
pub struct GetInitArg {}

#[derive(CandidType, Deserialize)]
pub struct GetInitResponse {
    pub init: Option<Init>,
}

#[derive(CandidType, Deserialize)]
pub struct GetLifecycleArg {}

#[derive(CandidType, Deserialize)]
pub struct GetLifecycleResponse {
    pub decentralization_sale_open_timestamp_seconds: Option<u64>,
    pub lifecycle: Option<i32>,
    pub decentralization_swap_termination_timestamp_seconds: Option<u64>,
}

#[derive(CandidType, Deserialize)]
pub struct GetOpenTicketArg {}

#[derive(CandidType, Deserialize)]
pub struct Icrc1Account {
    pub owner: Option<Principal>,
    pub subaccount: Option<serde_bytes::ByteBuf>,
}

#[derive(CandidType, Deserialize)]
pub struct Ticket {
    pub creation_time: u64,
    pub ticket_id: u64,
    pub account: Option<Icrc1Account>,
    pub amount_icp_e8s: u64,
}

#[derive(CandidType, Deserialize)]
pub struct Ok2 {
    pub ticket: Option<Ticket>,
}

#[derive(CandidType, Deserialize)]
pub struct Err1 {
    pub error_type: Option<i32>,
}

#[derive(CandidType, Deserialize)]
pub enum Result1 {
    Ok(Ok2),
    Err(Err1),
}

#[derive(CandidType, Deserialize)]
pub struct GetOpenTicketResponse {
    pub result: Option<Result1>,
}

#[derive(CandidType, Deserialize)]
pub struct GetSaleParametersArg {}

#[derive(CandidType, Deserialize)]
pub struct Params {
    pub min_participant_icp_e8s: u64,
    pub neuron_basket_construction_parameters: Option<NeuronBasketConstructionParameters>,
    pub max_icp_e8s: u64,
    pub swap_due_timestamp_seconds: u64,
    pub min_participants: u32,
    pub sns_token_e8s: u64,
    pub sale_delay_seconds: Option<u64>,
    pub max_participant_icp_e8s: u64,
    pub min_direct_participation_icp_e8s: Option<u64>,
    pub min_icp_e8s: u64,
    pub max_direct_participation_icp_e8s: Option<u64>,
}

#[derive(CandidType, Deserialize)]
pub struct GetSaleParametersResponse {
    pub params: Option<Params>,
}

#[derive(CandidType, Deserialize)]
pub struct GetStateArg {}

#[derive(CandidType, Deserialize)]
pub struct NeuronId {
    pub id: serde_bytes::ByteBuf,
}

#[derive(CandidType, Deserialize)]
pub struct NeuronAttributes {
    pub dissolve_delay_seconds: u64,
    pub memo: u64,
    pub followees: Vec<NeuronId>,
}

#[derive(CandidType, Deserialize)]
pub struct Principals {
    pub principals: Vec<Principal>,
}

#[derive(CandidType, Deserialize)]
pub struct CfInvestment {
    pub controller: Option<Principal>,
    pub hotkey_principal: String,
    pub hotkeys: Option<Principals>,
    pub nns_neuron_id: u64,
}

#[derive(CandidType, Deserialize)]
pub struct DirectInvestment {
    pub buyer_principal: String,
}

#[derive(CandidType, Deserialize)]
pub enum Investor {
    CommunityFund(CfInvestment),
    Direct(DirectInvestment),
}

#[derive(CandidType, Deserialize)]
pub struct SnsNeuronRecipe {
    pub sns: Option<TransferableAmount>,
    pub claimed_status: Option<i32>,
    pub neuron_attributes: Option<NeuronAttributes>,
    pub investor: Option<Investor>,
}

#[derive(CandidType, Deserialize)]
pub struct Timers {
    pub last_spawned_timestamp_seconds: Option<u64>,
    pub last_reset_timestamp_seconds: Option<u64>,
    pub requires_periodic_tasks: Option<bool>,
}

#[derive(CandidType, Deserialize)]
pub struct CfNeuron {
    pub has_created_neuron_recipes: Option<bool>,
    pub hotkeys: Option<Principals>,
    pub nns_neuron_id: u64,
    pub amount_icp_e8s: u64,
}

#[derive(CandidType, Deserialize)]
pub struct CfParticipant {
    pub controller: Option<Principal>,
    pub hotkey_principal: String,
    pub cf_neurons: Vec<CfNeuron>,
}

#[derive(CandidType, Deserialize)]
pub struct Swap {
    pub auto_finalize_swap_response: Option<FinalizeSwapResponse>,
    pub neuron_recipes: Vec<SnsNeuronRecipe>,
    pub next_ticket_id: Option<u64>,
    pub decentralization_sale_open_timestamp_seconds: Option<u64>,
    pub finalize_swap_in_progress: Option<bool>,
    pub timers: Option<Timers>,
    pub cf_participants: Vec<CfParticipant>,
    pub init: Option<Init>,
    pub already_tried_to_auto_finalize: Option<bool>,
    pub neurons_fund_participation_icp_e8s: Option<u64>,
    pub purge_old_tickets_last_completion_timestamp_nanoseconds: Option<u64>,
    pub direct_participation_icp_e8s: Option<u64>,
    pub lifecycle: i32,
    pub purge_old_tickets_next_principal: Option<serde_bytes::ByteBuf>,
    pub decentralization_swap_termination_timestamp_seconds: Option<u64>,
    pub buyers: Vec<(String, BuyerState)>,
    pub params: Option<Params>,
    pub open_sns_token_swap_proposal_id: Option<u64>,
}

#[derive(CandidType, Deserialize)]
pub struct DerivedState {
    pub sns_tokens_per_icp: f32,
    pub buyer_total_icp_e8s: u64,
    pub cf_participant_count: Option<u64>,
    pub neurons_fund_participation_icp_e8s: Option<u64>,
    pub direct_participation_icp_e8s: Option<u64>,
    pub direct_participant_count: Option<u64>,
    pub cf_neuron_count: Option<u64>,
}

#[derive(CandidType, Deserialize)]
pub struct GetStateResponse {
    pub swap: Option<Swap>,
    pub derived: Option<DerivedState>,
}

#[derive(CandidType, Deserialize)]
pub struct GetTimersArg {}

#[derive(CandidType, Deserialize)]
pub struct GetTimersResponse {
    pub timers: Option<Timers>,
}

#[derive(CandidType, Deserialize)]
pub struct ListCommunityFundParticipantsRequest {
    pub offset: Option<u64>,
    pub limit: Option<u32>,
}

#[derive(CandidType, Deserialize)]
pub struct ListCommunityFundParticipantsResponse {
    pub cf_participants: Vec<CfParticipant>,
}

#[derive(CandidType, Deserialize)]
pub struct ListDirectParticipantsRequest {
    pub offset: Option<u32>,
    pub limit: Option<u32>,
}

#[derive(CandidType, Deserialize)]
pub struct Participant {
    pub participation: Option<BuyerState>,
    pub participant_id: Option<Principal>,
}

#[derive(CandidType, Deserialize)]
pub struct ListDirectParticipantsResponse {
    pub participants: Vec<Participant>,
}

#[derive(CandidType, Deserialize)]
pub struct ListSnsNeuronRecipesRequest {
    pub offset: Option<u64>,
    pub limit: Option<u32>,
}

#[derive(CandidType, Deserialize)]
pub struct ListSnsNeuronRecipesResponse {
    pub sns_neuron_recipes: Vec<SnsNeuronRecipe>,
}

#[derive(CandidType, Deserialize)]
pub struct NewSaleTicketRequest {
    pub subaccount: Option<serde_bytes::ByteBuf>,
    pub amount_icp_e8s: u64,
}

#[derive(CandidType, Deserialize)]
pub struct InvalidUserAmount {
    pub min_amount_icp_e8s_included: u64,
    pub max_amount_icp_e8s_included: u64,
}

#[derive(CandidType, Deserialize)]
pub struct Err2 {
    pub invalid_user_amount: Option<InvalidUserAmount>,
    pub existing_ticket: Option<Ticket>,
    pub error_type: i32,
}

#[derive(CandidType, Deserialize)]
pub enum Result2 {
    Ok(Ok2),
    Err(Err2),
}

#[derive(CandidType, Deserialize)]
pub struct NewSaleTicketResponse {
    pub result: Option<Result2>,
}

#[derive(CandidType, Deserialize)]
pub struct NotifyPaymentFailureArg {}

#[derive(CandidType, Deserialize)]
pub struct RefreshBuyerTokensRequest {
    pub confirmation_text: Option<String>,
    pub buyer: String,
}

#[derive(CandidType, Deserialize)]
pub struct RefreshBuyerTokensResponse {
    pub icp_accepted_participation_e8s: u64,
    pub icp_ledger_account_balance_e8s: u64,
}

#[derive(CandidType, Deserialize)]
pub struct ResetTimersArg {}

#[derive(CandidType, Deserialize)]
pub struct ResetTimersRet {}
