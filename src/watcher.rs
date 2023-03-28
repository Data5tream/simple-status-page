use chrono::{Local, Utc};
use serde::{Deserialize};
use tokio_schedule::{every, Job};

#[derive(Deserialize)]
pub struct Watchpoint {
    name: String,
    ip: String,
    url: String
}

async fn run_watcher(watchlist: &Vec<Watchpoint>) -> Result<(), Box<dyn std::error::Error>> {
    println!("Running watchers - {:?}", Local::now());

    for wp in watchlist {
        println!(" - Run watcher for {} - {}", wp.name, wp.ip);
        let res = reqwest::get(&wp.url).await;
        match res {
            Ok(response) => {
                if response.status().is_success() {
                    println!("   [+] {} ({}) is up", wp.name, wp.url);
                } else {
                    println!("   [-] {} ({}) returned status code {}", wp.name, wp.url, response.status().as_str());
                }
            }
            Err(err) => {
                println!("   [-] Unable to connect to {} ({})", wp.name, wp.url);
                println!("       {:?}", err);
            }
        }
    }

    Ok(())
}

pub async fn start_watcher(interval: u32, watchlist: &Vec<Watchpoint>) {
    // run initial scan
    run_watcher(watchlist).await.expect("unable to perform checks");

    let every_second = every(interval)
        .seconds()
        .in_timezone(&Utc)
        .perform(|| async { run_watcher(watchlist).await.expect("unable to perform checks"); });
    every_second.await;
}