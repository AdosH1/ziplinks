use config::{Config, ConfigError, File};
use serde::Deserialize;

pub const CONFIG_FILE_PATH: &str = "./config/Default.toml";

#[derive(Debug, Deserialize, Clone)]
pub struct Server {
    pub port: usize,
    pub num_threads: usize,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub server: Server,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let s = Config::builder()
            .add_source(File::with_name(CONFIG_FILE_PATH))
            .build()?;
        s.try_deserialize()
    }
}
