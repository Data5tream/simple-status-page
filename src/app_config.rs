use std::sync::OnceLock;

use config::{Config, ConfigError};

pub struct ListenConfig {
    pub host: String,
    pub port: u16,
    pub url: String,
}

/// Get config from memory
pub fn get_config() -> &'static Config {
    static CONFIG: OnceLock<Config> = OnceLock::new();
    CONFIG.get_or_init(|| {
        Config::builder()
            .add_source(config::File::with_name("config").required(false))
            // Add in settings from the environment (with a prefix of APP)
            // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
            .add_source(config::Environment::with_prefix("APP").separator("_"))
            .build()
            .expect("Invalid or missing config file")
    })
}

pub fn load_listener_config() -> Result<ListenConfig, ConfigError> {
    let settings = get_config();

    Ok(ListenConfig {
        host: settings.get_string("webserver.host")?,
        port: settings.get::<u16>("webserver.port")?,
        url: settings.get_string("webserver.url")?,
    })
}
