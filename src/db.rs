use sled::Db;
use std::sync::LazyLock;

static DB_PATH: &str = "./db";
pub static DB: LazyLock<Db> = LazyLock::new(|| sled::open(DB_PATH).expect("unable to open db"));
