use std::fs;

use serde::{Deserialize, Serialize};

use crate::error::CatResult;

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub token: String,
}
impl Config {
    pub fn read_config() -> CatResult<Self> {
        let file = fs::read_to_string("./Config.toml")?;
        Ok(toml::from_str(&file)?)
    }
}
