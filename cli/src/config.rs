use serde::{Deserialize, Serialize};

use crate::constants::APP_NAME;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Config {
    pub device_token: Option<String>,
    pub backend_url: String,
    pub active: bool,
}

impl Config {
    pub fn load() -> Result<Config, Box<dyn std::error::Error>> {
        Ok(confy::load(APP_NAME, None)?)
    }

    pub fn save(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
        Ok(confy::store(APP_NAME, None, config)?)
    }
}
