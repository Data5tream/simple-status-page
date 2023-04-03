use chrono::{Local, Utc};
use redis::Commands;
use serde::{Deserialize, Serialize};
use tokio_schedule::{every, Job};

use crate::cache::get_redis_connection;

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

async fn run_watcher(watchlist: &Vec<Watchpoint>) -> Result<(), Box<dyn std::error::Error>> {
    println!("Running watchers - {:?}", Local::now());

    let mut con = get_redis_connection();

    for wp in watchlist {
        println!(" - Run watcher for {} - {}", wp.name, wp.ip);
        let res = reqwest::get(&wp.url).await;
        match res {
            Ok(response) => {
                con.set(
                    format!("status:{}:status-code", wp.id),
                    response.status().as_u16(),
                )?;
            }
            Err(_err) => {
                con.set(format!("status:{}:status-code", wp.id), 999)?;
            }
        }
    }

    Ok(())
}

pub async fn start_watcher(interval: u32, watchlist: &Vec<Watchpoint>) {
    // run initial scan
    run_watcher(watchlist)
        .await
        .expect("unable to perform checks");

    let every_second = every(interval)
        .seconds()
        .in_timezone(&Utc)
        .perform(|| async {
            run_watcher(watchlist)
                .await
                .expect("unable to perform checks");
        });
    every_second.await;
}
