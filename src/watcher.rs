use actix_rt::time::sleep;
use redis::Commands;
use serde::{Deserialize, Serialize};
use std::time::Duration;

use crate::cache::{get_redis_connection, get_watchpoints};
use crate::get_config;

#[derive(Deserialize, Serialize)]
pub struct Watchpoint {
    pub id: String,
    name: String,
    ip: String,
    url: String,
}

#[derive(Deserialize, Serialize)]
pub struct WatchpointStatus {
    pub watchpoint: Watchpoint,
    pub status: u16,
}

/// Check a watchpoints status and save it to redis
async fn check_watchpoint(wp: Watchpoint) {
    println!(" - Run watcher for {} - {}", wp.name, wp.ip);

    let mut con = get_redis_connection();

    let res = reqwest::get(&wp.url).await;
    match res {
        Ok(response) => {
            let _: () = con
                .set(
                    format!("status:{}:status-code", wp.id),
                    response.status().as_u16(),
                )
                .unwrap();
        }
        Err(_err) => {
            let _: () = con
                .set(format!("status:{}:status-code", wp.id), 999)
                .unwrap();
        }
    };
}

/// Get config from redis and ran watchpoints in parallel
async fn cron_job() {
    let settings = get_config();

    loop {
        // Get interval from cache
        let interval = settings
            .get::<u64>("watcher.interval")
            .expect("Invalid interval");

        // Run watcher in separate thread
        actix_rt::spawn(async {
            // Get watchpoints and run watcher
            let watchpoints = get_watchpoints().unwrap();
            for wp in watchpoints {
                actix_rt::spawn(async move {
                    check_watchpoint(wp).await;
                });
            }
        });

        // Wait for next execution
        sleep(Duration::from_secs(interval)).await;
    }
}

/// Set up the watcher thread
pub fn setup_watcher() {
    actix_rt::spawn(async {
        cron_job().await;
    });
}
