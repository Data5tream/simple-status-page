use log::{error, warn};
use redis::{Commands, Connection};

use crate::get_config;
use crate::watcher::{Watchpoint, WatchpointStatus};

/// Get a redis connection using the URL from the config file
pub fn get_redis_connection() -> Connection {
    let settings = get_config();
    let redis_url = settings
        .get_string("redis.url")
        .expect("unable to get redis string!");

    match redis::Client::open(redis_url)
        .expect("unable create redis client")
        .get_connection()
    {
        Ok(con) => con,
        Err(_) => {
            let msg = "Unable to get redis connection";
            error!("{}", msg);
            panic!("{}", msg)
        }
    }
}

/// Load the app configuration into memory (redis)
pub fn load_config() -> bool {
    let settings = get_config();

    let raw_watchlist = settings
        .get_array("watcher.watchlist")
        .expect("Invalid watch list");

    // Make sure we have something to watch
    if raw_watchlist.is_empty() {
        warn!("No entries in watchlist! Nothing to do");
        return false;
    }

    let mut watchlist: Vec<Watchpoint> = Vec::new();
    for i in &raw_watchlist {
        watchlist.push(i.clone().try_deserialize().expect("invalid config value"));
    }

    let mut con = get_redis_connection();
    let _: () = con
        .set(
            "config:watchpoints",
            serde_json::to_string(&watchlist).unwrap(),
        )
        .unwrap();

    true
}

/// Get a vector with all registered watchpoints
pub fn get_watchpoints() -> Result<Vec<Watchpoint>, ()> {
    let mut con = get_redis_connection();
    match con.get::<&str, String>("config:watchpoints") {
        Ok(k) => Ok(serde_json::from_str(k.as_str()).unwrap()),
        Err(_e) => Err(()),
    }
}

/// Get a vector with all registered watchpoints and their status
pub fn get_watchpoint_status() -> Result<Vec<WatchpointStatus>, ()> {
    let mut con = get_redis_connection();
    let watchpoints = match get_watchpoints() {
        Ok(d) => d,
        Err(()) => return Err(()),
    };

    let mut data: Vec<WatchpointStatus> = Vec::new();
    for watchpoint in watchpoints {
        let status = con
            .get::<&str, u16>(format!("status:{}:status-code", watchpoint.id).as_str())
            .unwrap();

        data.push(WatchpointStatus { watchpoint, status });
    }

    Ok(data)
}
