use log::warn;

use crate::db::DB;
use crate::get_config;
use crate::watcher::{Watchpoint, WatchpointStatus};

/// Load the app configuration into memory
pub fn load_watchers() -> Result<(), ()> {
    let settings = get_config();

    let raw_watchlist = settings
        .get_array("watcher.watchlist")
        .expect("Invalid watch list");

    // Make sure we have something to watch
    if raw_watchlist.is_empty() {
        warn!("No entries in watchlist! Nothing to do");
        return Err(());
    }

    let mut watchlist: Vec<Watchpoint> = Vec::new();
    for i in &raw_watchlist {
        watchlist.push(i.clone().try_deserialize().expect("invalid config value"));
    }

    let watchpoint_tree = DB
        .open_tree("watchpoints")
        .expect("failed to open watchpoints!");

    for w in watchlist {
        watchpoint_tree
            .insert(&w.id, bincode::serialize(&w).expect("serialization error"))
            .expect("failed to insert into db");
    }

    Ok(())
}

/// Get a vector with all registered watchpoints
pub fn get_watchpoints() -> Result<Vec<Watchpoint>, ()> {
    let watchpoint_tree = DB
        .open_tree("watchpoints")
        .expect("failed to open watchpoints!");

    let mut watchpoints: Vec<Watchpoint> = Vec::new();
    let iter = watchpoint_tree.iter();
    for value in iter {
        let data_vec = value.unwrap().1;
        watchpoints.push(bincode::deserialize(&data_vec).unwrap());
    }

    if watchpoints.is_empty() {
        Err(())
    } else {
        Ok(watchpoints)
    }
}

/// Get a vector with all registered watchpoints and their status
pub fn get_watchpoint_status() -> Result<Vec<WatchpointStatus>, ()> {
    let Ok(watchpoints) = get_watchpoints() else {
        return Err(());
    };

    let mut data: Vec<WatchpointStatus> = Vec::new();
    let status_tree = DB
        .open_tree("watchpoint-status")
        .expect("failed to open watchpoint-status!");

    for watchpoint in watchpoints {
        let status = match status_tree.get(&watchpoint.id) {
            Ok(d) => {
                if let Some(vec) = d {
                    // handle unknown status
                    bincode::deserialize::<u16>(&vec).unwrap_or_else(|_| {
                        warn!("failed to deserialize status for watchpoint: {watchpoint}");
                        995
                    })
                } else {
                    warn!("watchpoint {watchpoint} has missing status");
                    990
                }
            }
            Err(_) => 999,
        };

        data.push(WatchpointStatus { watchpoint, status });
    }

    Ok(data)
}
