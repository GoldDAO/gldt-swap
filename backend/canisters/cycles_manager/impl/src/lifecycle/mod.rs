use crate::State;

pub mod init;
mod post_upgrade;
mod pre_upgrade;

pub use init::*;

pub fn init_canister(state: State) {
    crate::jobs::start();
    crate::state::init_state(state);
}