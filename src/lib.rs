use config::Config;
use log::warn;
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;

use crate::app_config::get_config;
use crate::cache::load_watchers;
use crate::watcher::setup_watcher;

mod app_config;
mod cache;
mod db;
pub mod endpoints;
mod watcher;


#[allow(clippy::result_unit_err)]
pub fn setup_app() -> Result<Config, ()> {
    setup_logger().expect("Error setting up logger!");

    // load settings into singleton
    let settings = get_config();

    // Load config file into cache, exit if no watchpoints are configured
    if load_watchers().is_err() {
        warn!("No watchpoints configured! See README.md for instructions");
        return Err(());
    }

    // Create watcher thread
    setup_watcher();

    Ok(settings)
}

/// Setup logger
fn setup_logger() -> Result<(), fern::InitError> {
    let config = get_config();

    let level_string = match config.get_string("log.level") {
        Ok(lvl) => lvl.to_uppercase(),
        Err(_) => "INFO".to_string(),
    };

    let log_path = match config.get_string("log.path") {
        Ok(path) => path,
        Err(_) => "log.log".to_string(),
    };

    let level = match &*level_string {
        "TRACE" => log::LevelFilter::Trace,
        "DEBUG" => log::LevelFilter::Debug,
        "INFO" => log::LevelFilter::Info,
        "WARN" => log::LevelFilter::Warn,
        "ERROR" => log::LevelFilter::Error,
        "OFF" => log::LevelFilter::Off,
        _ => log::LevelFilter::Info,
    };

    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{}] [{}] {}: {}",
                OffsetDateTime::now_utc().format(&Rfc3339).unwrap(),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(level)
        .chain(std::io::stdout())
        .chain(fern::log_file(log_path)?)
        .apply()?;
    Ok(())
}
