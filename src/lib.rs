pub mod config;
pub mod database;
pub mod error;
pub mod events;
pub mod prelude;
pub mod queries;
pub mod utils;

use crate::prelude::*;

pub async fn start_bot() -> KittyResult {
    // dotenv::dotenv().map_err(ConfigError::CannotLoadEnviroment)?;
    let config = Config::load_or_parse()?;
    info!("Loaded config");
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::DIRECT_MESSAGES;

    let mut client = ClientBuilder::new(&config.token, intents)
        .event_handler(Events)
        .await
        .map_err(DiscordError::CannotBuildBot)?;

    {
        let mut pen = client.data.write().await;
        let database = Database::connect_to_database(config.database).await?;
        pen.insert::<Database>(database);
    }

    client.start().await.map_err(DiscordError::CannotStartBot)?;

    Ok(())
}
