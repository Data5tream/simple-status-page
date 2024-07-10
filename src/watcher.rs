use actix_rt::time::sleep;
use log::{debug, error};
use redis::{Commands, Connection};
use serde::{Deserialize, Serialize};
use std::time::Duration;

use crate::cache::{get_redis_connection, get_watchpoints};
use crate::get_config;

#[derive(Deserialize, Serialize)]
pub struct Watchpoint {
    pub id: String,
    name: String,
    kind: String,
    target: String,
    keyword: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct WatchpointStatus {
    pub watchpoint: Watchpoint,
    pub status: u16,
}

/// Check a URL endpoint
async fn check_url_endpoint(wp: &Watchpoint, con: &mut Connection) -> bool {
    let res = reqwest::get(&wp.target).await;
    match res {
        Ok(response) => {
            let _: () = con
                .set(
                    format!("status:{}:status-code", wp.id),
                    response.status().as_u16(),
                )
                .unwrap();

            true
        }
        Err(err) => {
            // Standard catch-all error code
            let mut status = 999;

            if err.is_connect() {
                // Handle connection errors
                let err_msg = err.to_string();
                if err_msg.contains(
                    "dns error: failed to lookup address information: Name or service not known",
                ) {
                    // DNS lookup failed
                    status = 600;
                } else if err_msg.contains("(unable to get local issuer certificate)") {
                    // TLS error
                    status = 601;
                }
            } else if err.is_status() {
                // Handle standard HTTP status codes
                status = err.status().unwrap().as_u16();
            }

            // Save status code to cache
            let _: () = con
                .set(format!("status:{}:status-code", wp.id), status)
                .unwrap();

            false
        }
    }
}

/// Check a keyword endpoint
async fn check_keyword_endpoint(wp: &Watchpoint, con: &mut Connection) -> bool {
    let keyword = match wp.keyword.as_ref() {
        None => {
            error!("invalid keywords configuration for watchpoint {}", wp.id);
            return false;
        }
        Some(kw) => kw,
    };

    let res = reqwest::get(&wp.target).await;
    match res {
        Ok(response) => {
            let body = response.text().await;

            match body {
                Ok(txt) => {
                    if txt.contains(keyword) {
                        let _: () = con
                            .set(format!("status:{}:status-code", wp.id), 200)
                            .unwrap();

                        true
                    } else {
                        let _: () = con
                            .set(format!("status:{}:status-code", wp.id), 604)
                            .unwrap();

                        false
                    }
                }
                Err(_) => {
                    let _: () = con
                        .set(format!("status:{}:status-code", wp.id), 610)
                        .unwrap();

                    false
                }
            }
        }
        Err(err) => {
            // Standard catch-all error code
            let mut status = 999;

            if err.is_connect() {
                // Handle connection errors
                let err_msg = err.to_string();
                if err_msg.contains(
                    "dns error: failed to lookup address information: Name or service not known",
                ) {
                    // DNS lookup failed
                    status = 600;
                } else if err_msg.contains("(unable to get local issuer certificate)") {
                    // TLS error
                    status = 601;
                }
            } else if err.is_status() {
                // Handle standard HTTP status codes
                status = err.status().unwrap().as_u16();
            }

            // Save status code to cache
            let _: () = con
                .set(format!("status:{}:status-code", wp.id), status)
                .unwrap();

            false
        }
    }
}

/// Check a watchpoints status and save it to redis
async fn check_watchpoint(wp: Watchpoint) {
    debug!(" - Run watcher of type {} for {}", wp.kind, wp.name);

    let mut con = get_redis_connection();

    if wp.kind == "url" {
        check_url_endpoint(&wp, &mut con).await;
    } else if wp.kind == "keyword" {
        check_keyword_endpoint(&wp, &mut con).await;
    }
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
