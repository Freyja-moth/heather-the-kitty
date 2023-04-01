mod commands;
mod config;
mod error;
mod events;

use config::Config;
use error::CatResult;

use events::Handler;
use serenity::Client;

#[tokio::main]
async fn main() -> CatResult<()> {
    let config = Config::read_config()?;

    let mut client = Client::builder(&config.token)
        .event_handler(Handler)
        .await?;

    client.start().await?;

    Ok(())
}
