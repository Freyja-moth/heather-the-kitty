use crate::error::CatResult;
use serde::{Deserialize, Serialize};
use std::fs;

const CONFIG_PATH: &str = "Config.toml";

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub token: String,
}
impl Config {
    pub fn read_config() -> CatResult<Self> {
        let file = fs::read_to_string(CONFIG_PATH)?;
        Ok(toml::from_str(&file)?)
    }
}
