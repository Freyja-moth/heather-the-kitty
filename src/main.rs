mod commands;
mod config;
mod error;
mod events;

use crate::commands::Commands;
use crate::config::Config;
use crate::error::CatResult;

use events::Handler;
use log::info;
use serenity::{prelude::GatewayIntents, Client};

#[tokio::main]
async fn main() -> CatResult<()> {
    env_logger::init();

    let config = Config::read_config()?;
    let intents = GatewayIntents::all();
    info!("Loaded config!");

    let mut client = Client::builder(&config.token, intents)
        .event_handler(Handler)
        .event_handler(Commands)
        .await?;

    client.start().await?;

    Ok(())
}
