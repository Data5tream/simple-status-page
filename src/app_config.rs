use std::sync::Mutex;

use config::Config;

static CONFIG: Mutex<Option<Config>> = Mutex::new(None);

/// Load config into memory
pub fn load_config() {
    let config = Config::builder()
        .add_source(config::File::with_name("config"))
        // Add in settings from the environment (with a prefix of APP)
        // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
        .add_source(config::Environment::with_prefix("APP").separator("_"))
        .build()
        .expect("Invalid or missing config file");

    let mut mutex = CONFIG.lock().unwrap();
    *mutex = Some(config);
}

/// Get config from memory
pub fn get_config() -> Config {
    let mut mutex = CONFIG.lock().unwrap();

    match &mut *mutex {
        Some(m) => m.clone(),
        None => {
            // drop the mutex so we can set it in load_config()
            drop(mutex);
            load_config();
            get_config()
        }
    }
}
