export const idlFactory = ({ IDL }) => {
  const BuildVersion = IDL.Record({
    major: IDL.Nat32,
    minor: IDL.Nat32,
    patch: IDL.Nat32,
  });
  const UpgradeArgs = IDL.Record({
    version: BuildVersion,
    commit_hash: IDL.Text,
  });
  const InitArgs = IDL.Record({
    test_mode: IDL.Bool,
    reward_types: IDL.Vec(
      IDL.Tuple(IDL.Text, IDL.Tuple(IDL.Principal, IDL.Nat))
    ),
    authorized_principals: IDL.Vec(IDL.Principal),
    version: BuildVersion,
    gld_sns_governance_canister_id: IDL.Principal,
    gldt_ledger_id: IDL.Principal,
    goldao_ledger_id: IDL.Principal,
    commit_hash: IDL.Text,
    gld_sns_rewards_canister_id: IDL.Principal,
  });
  const Args_6 = IDL.Variant({ Upgrade: UpgradeArgs, Init: InitArgs });
  const Args = IDL.Record({ id: IDL.Nat64, token: IDL.Text });
  const Duration = IDL.Record({ secs: IDL.Nat64, nanos: IDL.Nat32 });
  const DissolveState = IDL.Variant({
    Dissolved: IDL.Null,
    Dissolving: IDL.Null,
    NotDissolving: IDL.Null,
  });
  const StakePositionResponse = IDL.Record({
    id: IDL.Nat64,
    staked: IDL.Nat,
    dissolve_delay: Duration,
    early_unstake_fee: IDL.Nat,
    claimable_rewards: IDL.Vec(IDL.Tuple(IDL.Text, IDL.Nat)),
    dissolved_date: IDL.Opt(IDL.Nat64),
    created_at: IDL.Nat64,
    age_bonus_multiplier: IDL.Float64,
    owned_by: IDL.Principal,
    dissolve_state: DissolveState,
    weighted_stake: IDL.Nat,
  });
  const ClaimRewardErrors = IDL.Variant({
    TransferError: IDL.Text,
    InvalidRewardToken: IDL.Text,
    AlreadyProcessing: IDL.Text,
    InvalidPrincipal: IDL.Text,
    NotFound: IDL.Text,
    NotAuthorized: IDL.Text,
    CallError: IDL.Text,
    TokenImbalance: IDL.Text,
  });
  const Result = IDL.Variant({
    Ok: StakePositionResponse,
    Err: ClaimRewardErrors,
  });
  const Args_1 = IDL.Record({ amount: IDL.Nat64 });
  const CreateNeuronError = IDL.Variant({
    TransferError: IDL.Text,
    InternalError: IDL.Text,
  });
  const Result_1 = IDL.Variant({
    Ok: IDL.Vec(IDL.Nat8),
    Err: CreateNeuronError,
  });
  const Args_2 = IDL.Record({ amount: IDL.Nat });
  const AddStakePositionErrors = IDL.Variant({
    MaxActiveStakePositions: IDL.Text,
    TransferError: IDL.Text,
    AlreadyProcessing: IDL.Text,
    InvalidStakeAmount: IDL.Text,
    InvalidPrincipal: IDL.Text,
    CanisterAtCapacity: IDL.Text,
    CallError: IDL.Text,
  });
  const Result_2 = IDL.Variant({
    Ok: StakePositionResponse,
    Err: AddStakePositionErrors,
  });
  const Args_3 = IDL.Record({
    limit: IDL.Opt(IDL.Nat64),
    starting_week: IDL.Nat64,
  });
  const ArchiveCanister = IDL.Record({
    active: IDL.Bool,
    canister_id: IDL.Principal,
  });
  const Args_4 = IDL.Record({
    user: IDL.Principal,
    limit: IDL.Nat64,
    start: IDL.Nat64,
  });
  const NeuronId = IDL.Record({ id: IDL.Vec(IDL.Nat8) });
  const NeuronPermission = IDL.Record({
    principal: IDL.Opt(IDL.Principal),
    permission_type: IDL.Vec(IDL.Int32),
  });
  const DissolveState_1 = IDL.Variant({
    DissolveDelaySeconds: IDL.Nat64,
    WhenDissolvedTimestampSeconds: IDL.Nat64,
  });
  const Subaccount = IDL.Record({ subaccount: IDL.Vec(IDL.Nat8) });
  const Account = IDL.Record({
    owner: IDL.Opt(IDL.Principal),
    subaccount: IDL.Opt(Subaccount),
  });
  const DisburseMaturityInProgress = IDL.Record({
    timestamp_of_disbursement_seconds: IDL.Nat64,
    amount_e8s: IDL.Nat64,
    account_to_disburse_to: IDL.Opt(Account),
  });
  const Followees = IDL.Record({ followees: IDL.Vec(NeuronId) });
  const Neuron = IDL.Record({
    id: IDL.Opt(NeuronId),
    staked_maturity_e8s_equivalent: IDL.Opt(IDL.Nat64),
    permissions: IDL.Vec(NeuronPermission),
    maturity_e8s_equivalent: IDL.Nat64,
    cached_neuron_stake_e8s: IDL.Nat64,
    created_timestamp_seconds: IDL.Nat64,
    source_nns_neuron_id: IDL.Opt(IDL.Nat64),
    auto_stake_maturity: IDL.Opt(IDL.Bool),
    aging_since_timestamp_seconds: IDL.Nat64,
    dissolve_state: IDL.Opt(DissolveState_1),
    voting_power_percentage_multiplier: IDL.Nat64,
    vesting_period_seconds: IDL.Opt(IDL.Nat64),
    disburse_maturity_in_progress: IDL.Vec(DisburseMaturityInProgress),
    followees: IDL.Vec(IDL.Tuple(IDL.Nat64, Followees)),
    neuron_fees_e8s: IDL.Nat64,
  });
  const RewardRoundStatus = IDL.Variant({
    RewardsClaimed: IDL.Null,
    RewardsAllocated: IDL.Null,
    AllocationInProgress: IDL.Null,
  });
  const RewardRound = IDL.Record({
    status: RewardRoundStatus,
    token_symbol: IDL.Text,
    created_at: IDL.Nat64,
    rewards: IDL.Nat,
  });
  const Split = IDL.Record({ memo: IDL.Nat64, amount_e8s: IDL.Nat64 });
  const Follow = IDL.Record({
    function_id: IDL.Nat64,
    followees: IDL.Vec(NeuronId),
  });
  const DisburseMaturity = IDL.Record({
    to_account: IDL.Opt(Account),
    percentage_to_disburse: IDL.Nat32,
  });
  const MemoAndController = IDL.Record({
    controller: IDL.Opt(IDL.Principal),
    memo: IDL.Nat64,
  });
  const By = IDL.Variant({
    MemoAndController: MemoAndController,
    NeuronId: IDL.Record({}),
  });
  const ClaimOrRefresh = IDL.Record({ by: IDL.Opt(By) });
  const ChangeAutoStakeMaturity = IDL.Record({
    requested_setting_for_auto_stake_maturity: IDL.Bool,
  });
  const IncreaseDissolveDelay = IDL.Record({
    additional_dissolve_delay_seconds: IDL.Nat32,
  });
  const SetDissolveTimestamp = IDL.Record({
    dissolve_timestamp_seconds: IDL.Nat64,
  });
  const Operation = IDL.Variant({
    ChangeAutoStakeMaturity: ChangeAutoStakeMaturity,
    StopDissolving: IDL.Record({}),
    StartDissolving: IDL.Record({}),
    IncreaseDissolveDelay: IncreaseDissolveDelay,
    SetDissolveTimestamp: SetDissolveTimestamp,
  });
  const Configure = IDL.Record({ operation: IDL.Opt(Operation) });
  const ProposalId = IDL.Record({ id: IDL.Nat64 });
  const RegisterVote = IDL.Record({
    vote: IDL.Int32,
    proposal: IDL.Opt(ProposalId),
  });
  const DefaultFollowees = IDL.Record({
    followees: IDL.Vec(IDL.Tuple(IDL.Nat64, Followees)),
  });
  const NeuronPermissionList = IDL.Record({
    permissions: IDL.Vec(IDL.Int32),
  });
  const VotingRewardsParameters = IDL.Record({
    final_reward_rate_basis_points: IDL.Opt(IDL.Nat64),
    initial_reward_rate_basis_points: IDL.Opt(IDL.Nat64),
    reward_rate_transition_duration_seconds: IDL.Opt(IDL.Nat64),
    round_duration_seconds: IDL.Opt(IDL.Nat64),
  });
  const NervousSystemParameters = IDL.Record({
    default_followees: IDL.Opt(DefaultFollowees),
    max_dissolve_delay_seconds: IDL.Opt(IDL.Nat64),
    max_dissolve_delay_bonus_percentage: IDL.Opt(IDL.Nat64),
    max_followees_per_function: IDL.Opt(IDL.Nat64),
    neuron_claimer_permissions: IDL.Opt(NeuronPermissionList),
    neuron_minimum_stake_e8s: IDL.Opt(IDL.Nat64),
    max_neuron_age_for_age_bonus: IDL.Opt(IDL.Nat64),
    initial_voting_period_seconds: IDL.Opt(IDL.Nat64),
    neuron_minimum_dissolve_delay_to_vote_seconds: IDL.Opt(IDL.Nat64),
    reject_cost_e8s: IDL.Opt(IDL.Nat64),
    max_proposals_to_keep_per_action: IDL.Opt(IDL.Nat32),
    wait_for_quiet_deadline_increase_seconds: IDL.Opt(IDL.Nat64),
    max_number_of_neurons: IDL.Opt(IDL.Nat64),
    transaction_fee_e8s: IDL.Opt(IDL.Nat64),
    max_number_of_proposals_with_ballots: IDL.Opt(IDL.Nat64),
    max_age_bonus_percentage: IDL.Opt(IDL.Nat64),
    neuron_grantable_permissions: IDL.Opt(NeuronPermissionList),
    voting_rewards_parameters: IDL.Opt(VotingRewardsParameters),
    maturity_modulation_disabled: IDL.Opt(IDL.Bool),
    max_number_of_principals_per_neuron: IDL.Opt(IDL.Nat64),
  });
  const GenericNervousSystemFunction = IDL.Record({
    validator_canister_id: IDL.Opt(IDL.Principal),
    target_canister_id: IDL.Opt(IDL.Principal),
    validator_method_name: IDL.Opt(IDL.Text),
    target_method_name: IDL.Opt(IDL.Text),
  });
  const FunctionType = IDL.Variant({
    NativeNervousSystemFunction: IDL.Record({}),
    GenericNervousSystemFunction: GenericNervousSystemFunction,
  });
  const NervousSystemFunction = IDL.Record({
    id: IDL.Nat64,
    name: IDL.Text,
    description: IDL.Opt(IDL.Text),
    function_type: IDL.Opt(FunctionType),
  });
  const RegisterDappCanisters = IDL.Record({
    canister_ids: IDL.Vec(IDL.Principal),
  });
  const TransferSnsTreasuryFunds = IDL.Record({
    from_treasury: IDL.Int32,
    to_principal: IDL.Opt(IDL.Principal),
    to_subaccount: IDL.Opt(Subaccount),
    memo: IDL.Opt(IDL.Nat64),
    amount_e8s: IDL.Nat64,
  });
  const UpgradeSnsControlledCanister = IDL.Record({
    new_canister_wasm: IDL.Vec(IDL.Nat8),
    mode: IDL.Opt(IDL.Int32),
    canister_id: IDL.Opt(IDL.Principal),
    canister_upgrade_arg: IDL.Opt(IDL.Vec(IDL.Nat8)),
  });
  const DeregisterDappCanisters = IDL.Record({
    canister_ids: IDL.Vec(IDL.Principal),
    new_controllers: IDL.Vec(IDL.Principal),
  });
  const ManageSnsMetadata = IDL.Record({
    url: IDL.Opt(IDL.Text),
    logo: IDL.Opt(IDL.Text),
    name: IDL.Opt(IDL.Text),
    description: IDL.Opt(IDL.Text),
  });
  const ExecuteGenericNervousSystemFunction = IDL.Record({
    function_id: IDL.Nat64,
    payload: IDL.Vec(IDL.Nat8),
  });
  const Motion = IDL.Record({ motion_text: IDL.Text });
  const Action = IDL.Variant({
    ManageNervousSystemParameters: NervousSystemParameters,
    AddGenericNervousSystemFunction: NervousSystemFunction,
    RemoveGenericNervousSystemFunction: IDL.Nat64,
    UpgradeSnsToNextVersion: IDL.Record({}),
    RegisterDappCanisters: RegisterDappCanisters,
    TransferSnsTreasuryFunds: TransferSnsTreasuryFunds,
    UpgradeSnsControlledCanister: UpgradeSnsControlledCanister,
    DeregisterDappCanisters: DeregisterDappCanisters,
    Unspecified: IDL.Record({}),
    ManageSnsMetadata: ManageSnsMetadata,
    ExecuteGenericNervousSystemFunction: ExecuteGenericNervousSystemFunction,
    Motion: Motion,
  });
  const Proposal = IDL.Record({
    url: IDL.Text,
    title: IDL.Text,
    action: IDL.Opt(Action),
    summary: IDL.Text,
  });
  const StakeMaturity = IDL.Record({
    percentage_to_stake: IDL.Opt(IDL.Nat32),
  });
  const RemoveNeuronPermissions = IDL.Record({
    permissions_to_remove: IDL.Opt(NeuronPermissionList),
    principal_id: IDL.Opt(IDL.Principal),
  });
  const AddNeuronPermissions = IDL.Record({
    permissions_to_add: IDL.Opt(NeuronPermissionList),
    principal_id: IDL.Opt(IDL.Principal),
  });
  const MergeMaturity = IDL.Record({ percentage_to_merge: IDL.Nat32 });
  const Amount = IDL.Record({ e8s: IDL.Nat64 });
  const Disburse = IDL.Record({
    to_account: IDL.Opt(Account),
    amount: IDL.Opt(Amount),
  });
  const Command = IDL.Variant({
    Split: Split,
    Follow: Follow,
    DisburseMaturity: DisburseMaturity,
    ClaimOrRefresh: ClaimOrRefresh,
    Configure: Configure,
    RegisterVote: RegisterVote,
    MakeProposal: Proposal,
    StakeMaturity: StakeMaturity,
    RemoveNeuronPermissions: RemoveNeuronPermissions,
    AddNeuronPermissions: AddNeuronPermissions,
    MergeMaturity: MergeMaturity,
    Disburse: Disburse,
  });
  const Args_5 = IDL.Record({
    command: Command,
    neuron_id: IDL.Vec(IDL.Nat8),
  });
  const Response = IDL.Variant({
    Success: IDL.Text,
    InternalError: IDL.Text,
  });
  const Result_3 = IDL.Variant({ Ok: IDL.Null, Err: IDL.Text });
  const Result_4 = IDL.Variant({ Ok: IDL.Text, Err: IDL.Text });
  const UnstakeErrors = IDL.Variant({
    NoDissolveDateSet: IDL.Text,
    AlreadyProcessing: IDL.Text,
    AlreadyUnstaked: IDL.Text,
    DissolveDateNotSatisfied: IDL.Text,
    InvalidDissolveState: IDL.Text,
    CantUnstakeWithRewardsBalance: IDL.Text,
  });
  const RemoveRewardErrors = IDL.Variant({
    InsufficientBalance: IDL.Text,
    RewardTokenTypeDoesNotExist: IDL.Text,
  });
  const StakePositionError = IDL.Variant({
    StartDissolvingError: IDL.Text,
    AddStakePositionError: AddStakePositionErrors,
    UnStakeError: UnstakeErrors,
    AddRewardError: IDL.Text,
    RemoveRewardError: RemoveRewardErrors,
  });
  const StartDissolvingErrors = IDL.Variant({
    InvalidPrincipal: IDL.Text,
    NotFound: IDL.Text,
    NotAuthorized: IDL.Text,
    StakePositionError: StakePositionError,
  });
  const Result_5 = IDL.Variant({
    Ok: StakePositionResponse,
    Err: StartDissolvingErrors,
  });
  const UnstakeRequestErrors = IDL.Variant({
    TransferError: IDL.Text,
    UnstakeErrors: UnstakeErrors,
    InvalidPrincipal: IDL.Text,
    NotFound: IDL.Text,
    AlreadyUnstaked: IDL.Text,
    NotAuthorized: IDL.Text,
    CallError: IDL.Text,
    InvalidState: IDL.Text,
  });
  const Result_6 = IDL.Variant({
    Ok: StakePositionResponse,
    Err: UnstakeRequestErrors,
  });
  const UnstakeEarlyRequestErrors = IDL.Variant({
    TransferError: IDL.Text,
    UnstakeErrors: UnstakeErrors,
    AlreadyProcessing: IDL.Text,
    AlreadyUnstakedEarly: IDL.Text,
    InvalidPrincipal: IDL.Text,
    NotFound: IDL.Text,
    NotAuthorized: IDL.Text,
    CallError: IDL.Text,
  });
  const Result_7 = IDL.Variant({
    Ok: StakePositionResponse,
    Err: UnstakeEarlyRequestErrors,
  });
  return IDL.Service({
    claim_reward: IDL.Func([Args], [Result], []),
    commit: IDL.Func([], [], []),
    create_neuron: IDL.Func([Args_1], [Result_1], []),
    create_stake_position: IDL.Func([Args_2], [Result_2], []),
    get_active_user_positions: IDL.Func(
      [IDL.Opt(IDL.Principal)],
      [IDL.Vec(StakePositionResponse)],
      ["query"]
    ),
    get_apy_overall: IDL.Func([IDL.Null], [IDL.Float64], ["query"]),
    get_apy_timeseries: IDL.Func(
      [Args_3],
      [IDL.Vec(IDL.Tuple(IDL.Nat64, IDL.Float64))],
      ["query"]
    ),
    get_archive_canisters: IDL.Func(
      [IDL.Null],
      [IDL.Vec(ArchiveCanister)],
      ["query"]
    ),
    get_historic_position_by_id: IDL.Func(
      [IDL.Nat64],
      [IDL.Opt(StakePositionResponse)],
      []
    ),
    get_historic_positions_by_user: IDL.Func(
      [Args_4],
      [IDL.Vec(StakePositionResponse)],
      []
    ),
    get_historic_positions_total_by_user: IDL.Func(
      [IDL.Opt(IDL.Principal)],
      [IDL.Nat64],
      []
    ),
    get_neurons: IDL.Func([IDL.Null], [IDL.Vec(Neuron)], ["query"]),
    get_position_by_id: IDL.Func(
      [IDL.Nat64],
      [IDL.Opt(StakePositionResponse)],
      ["query"]
    ),
    get_reward_rounds: IDL.Func([IDL.Null], [IDL.Vec(RewardRound)], ["query"]),
    get_total_allocated_rewards: IDL.Func(
      [IDL.Null],
      [IDL.Vec(IDL.Tuple(IDL.Text, IDL.Nat))],
      ["query"]
    ),
    get_total_staked: IDL.Func([IDL.Null], [IDL.Nat], ["query"]),
    manage_sns_neuron: IDL.Func([Args_5], [Response], []),
    manual_sync_neurons: IDL.Func([IDL.Null], [Result_3], []),
    process_oldest_reward_round: IDL.Func([IDL.Null], [Result_4], []),
    start_dissolving: IDL.Func([IDL.Nat64], [Result_5], []),
    unstake: IDL.Func([IDL.Nat64], [Result_6], []),
    unstake_early: IDL.Func([IDL.Nat64], [Result_7], []),
  });
};
export const init = ({ IDL }) => {
  const BuildVersion = IDL.Record({
    major: IDL.Nat32,
    minor: IDL.Nat32,
    patch: IDL.Nat32,
  });
  const UpgradeArgs = IDL.Record({
    version: BuildVersion,
    commit_hash: IDL.Text,
  });
  const InitArgs = IDL.Record({
    test_mode: IDL.Bool,
    reward_types: IDL.Vec(
      IDL.Tuple(IDL.Text, IDL.Tuple(IDL.Principal, IDL.Nat))
    ),
    authorized_principals: IDL.Vec(IDL.Principal),
    version: BuildVersion,
    gld_sns_governance_canister_id: IDL.Principal,
    gldt_ledger_id: IDL.Principal,
    goldao_ledger_id: IDL.Principal,
    commit_hash: IDL.Text,
    gld_sns_rewards_canister_id: IDL.Principal,
  });
  const Args_6 = IDL.Variant({ Upgrade: UpgradeArgs, Init: InitArgs });
  return [Args_6];
};
