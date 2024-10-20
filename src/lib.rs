/*
   Simple Status Page - a simple service status app built with rust
   Copyright (C) 2023-2024  Simon Stefan Barth

   This program is free software: you can redistribute it and/or modify
   it under the terms of the GNU Affero General Public License as published
   by the Free Software Foundation, either version 3 of the License, or
   (at your option) any later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU Affero General Public License for more details.

   You should have received a copy of the GNU Affero General Public License
   along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/
use config::Config;
use log::warn;
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;

use crate::app_config::get_config;

mod app_config;
mod cache;
mod db;
pub mod endpoints;
mod watcher;

/// Set up the logger and the watcher threads
///
/// # Panics
///
/// panics when the logger cannot be created
///
/// # Errors
///
/// returns an error when no watchpoints are configured
#[allow(clippy::result_unit_err)]
pub fn setup_app() -> Result<Config, ()> {
    setup_logger().expect("Error setting up logger!");

    // load settings into singleton
    let settings = get_config();

    // Load config file into cache, exit if no watchpoints are configured
    if cache::load_watchers().is_err() {
        warn!("No watchpoints configured! See README.md for instructions");
        return Err(());
    }

    // Create watcher thread
    watcher::setup();

    Ok(settings)
}

/// Setup logger
fn setup_logger() -> Result<(), fern::InitError> {
    let config = get_config();

    let level_string = match config.get_string("log.level") {
        Ok(lvl) => lvl.to_uppercase(),
        Err(_) => "INFO".to_string(),
    };

    let log_path = config
        .get_string("log.path")
        .unwrap_or_else(|_| "log.log".to_string());

    #[allow(clippy::wildcard_in_or_patterns)]
    let level = match &*level_string {
        "TRACE" => log::LevelFilter::Trace,
        "DEBUG" => log::LevelFilter::Debug,
        "WARN" => log::LevelFilter::Warn,
        "ERROR" => log::LevelFilter::Error,
        "OFF" => log::LevelFilter::Off,
        "INFO" | _ => log::LevelFilter::Info,
    };

    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{}] [{}] {}: {}",
                OffsetDateTime::now_utc().format(&Rfc3339).unwrap(),
                record.target(),
                record.level(),
                message
            ));
        })
        .level(level)
        .chain(std::io::stdout())
        .chain(fern::log_file(log_path)?)
        .apply()?;
    Ok(())
}
