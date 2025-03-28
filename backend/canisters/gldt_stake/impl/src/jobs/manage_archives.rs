use std::time::Duration;

use crate::{
    model::archive_system::{
        check_storage_and_create_archive, create_archive_canister, update_archive_canisters,
    },
    state::{mutate_state, read_state},
};
use canister_time::{run_interval, run_once};
use gldt_stake_common::{
    archive::ArchiveCanister,
    archive::{ArchiveDownReason, ArchiveStatus, MANAGE_NEW_ARCHIVES_INTERVAL},
};
use ic_cdk::trap;
use tracing::{debug, info};

pub fn start_job() {
    run_once(spawn_archive_on_init);
    run_interval(
        Duration::from_millis(MANAGE_NEW_ARCHIVES_INTERVAL),
        spawn_manage_archives,
    );
}

pub fn spawn_archive_on_init() {
    ic_cdk::spawn(archive_on_init())
}

async fn archive_on_init() {
    let num_archive_canisters = read_state(|s| s.data.archive_system.get_total_archive_canisters());
    if num_archive_canisters == 0 {
        match create_archive_canister().await {
            Ok(principal) => {
                mutate_state(|s| {
                    s.data
                        .archive_system
                        .set_new_archive_canister(ArchiveCanister {
                            canister_id: principal,
                            active: true,
                        });
                    s.data.archive_system.set_archive_status(ArchiveStatus::Up)
                });

                info!("SUCCESS:: initial archive canister created : {principal:?}");
            }
            Err(e) => {
                mutate_state(|s| {
                    s.data
                        .archive_system
                        .set_archive_status(ArchiveStatus::Down(
                            ArchiveDownReason::NewArchiveError(e.clone()),
                        ));
                });

                trap(&format!("{e:?}"));
            }
        }
        return;
    }

    match update_archive_canisters().await {
        Ok(_) => {
            info!("SUCCESS : archive upgrade - all archives upgraded successfully");
            mutate_state(|s| s.data.archive_system.set_archive_status(ArchiveStatus::Up));
        }
        Err(errors) => {
            for e in errors {
                debug!(e);
                mutate_state(|s| {
                    s.data
                        .archive_system
                        .set_archive_status(ArchiveStatus::Down(
                            ArchiveDownReason::UpgradingArchivesFailed(e),
                        ))
                });
            }
        }
    }
}

pub fn spawn_manage_archives() {
    let is_running = read_state(|s| s.data.is_archive_cron_running);
    if is_running {
        return;
    }
    ic_cdk::spawn(manage_archives())
}

pub async fn manage_archives() {
    mutate_state(|s| {
        s.data.is_archive_cron_running = true;
    });
    let _ = check_storage_and_create_archive().await;
    mutate_state(|s| {
        s.data.is_archive_cron_running = false;
    });
}
