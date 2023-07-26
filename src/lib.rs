use config::Config;
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;

/// Get config from config file and environmental variables
pub fn get_config() -> Config {
    Config::builder()
        .add_source(config::File::with_name("config"))
        // Add in settings from the environment (with a prefix of APP)
        // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
        .add_source(config::Environment::with_prefix("APP").separator("_"))
        .build()
        .expect("Invalid or missing config file")
}

/// Setup logger
pub fn setup_logger() -> Result<(), fern::InitError> {
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
