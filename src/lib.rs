use config::Config;

/// Get config from config file and environmental variables
pub fn get_config() -> Config {
    Config::builder()
        .add_source(config::File::with_name("config"))
        // Add in settings from the environment (with a prefix of APP)
        // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
        .add_source(config::Environment::with_prefix("APP"))
        .build()
        .expect("Invalid or missing config file")
}
