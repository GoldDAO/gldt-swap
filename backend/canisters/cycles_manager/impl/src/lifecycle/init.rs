use crate::lifecycle::init_canister;
use crate::Data;
use canister_tracing_macros::trace;
pub use cycles_manager_api_canister::init::InitArgs;
use ic_cdk_macros::init;
use tracing::info;
use utils::env::CanisterEnv;
use utils::env::Environment;

#[init]
#[trace]
fn init(args: InitArgs) {
    canister_logger::init(args.test_mode);
    let env = CanisterEnv::new(args.test_mode);

    let data = Data::new(
        args.authorized_principals,
        args.canisters,
        args.sns_root_canister,
        args.max_top_up_amount,
        args.min_cycles_balance,
        env.now(),
    );

    let state = crate::State::new(env, data);
    init_canister(state);

    info!("Initialization complete");
}