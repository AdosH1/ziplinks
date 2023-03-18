use config::{Config, ConfigError, File};
use serde::{Deserialize, Serialize};

pub const CONFIG_FILE_PATH: &str = "./config/default.toml";

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Server {
    pub port: usize,
    pub num_threads: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum DatabaseType {
    Memory,
    Redis
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Database {
    pub address: String,
    pub database_type: DatabaseType
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Settings {
    pub server: Server,
    pub database: Database
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let s = Config::builder()
            .add_source(File::with_name(CONFIG_FILE_PATH))
            .build()?;
        s.try_deserialize()
    }
}
