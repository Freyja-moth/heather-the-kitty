use crate::prelude::*;

#[derive(Parser, Deserialize, Serialize, Debug)]
#[command(author, version, about, long_about = Some("Config can either be loaded from Config.toml or passes in from the command line"))]
pub struct Config {
    #[arg(short, long)]
    pub token: String,
    #[arg(short, long)]
    pub database: String,
}
impl Config {
    /// Loads heathers config from Config.toml
    ///
    /// # Errors
    /// [ConfigError::CannotLoadConfig]: If there has been an error while loading the config
    /// [ConfigError::CannotParseConfig]: If the config that has been loaded, cannot be parsed
    fn load() -> KittyResult<Self> {
        Ok(read_to_string("Config.toml")
            .map_err(ConfigError::CannotLoadConfig)
            .and_then(|config| toml::from_str(&config).map_err(ConfigError::CannotParseConfig))?)
    }
    /// Loads or parses the config
    ///
    /// If config cannot be loaded then it will try and parse it from the commands inputed
    pub fn load_or_parse() -> KittyResult<Self> {
        let Ok(config) = Config::load() else {
            return Ok(Config::parse());
        };
        Ok(config)
    }
}
