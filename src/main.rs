mod commands;
mod error;
mod events;

use commands::GENERAL_GROUP;
use error::*;
use events::Handler;
use rand::Rng;

use std::env;

use serenity::{framework::standard::StandardFramework, prelude::GatewayIntents, Client};

fn read_token() -> CatResult<String> {
    env::var("catbot_token").map_err(|_| CatError::CannotReadToken)
}

fn chance() -> f64 {
    (rand::thread_rng().gen::<f64>() * 100.0).round() / 100.0
}

#[tokio::main]
async fn main() -> CatResult<()> {
    env_logger::init();
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("~"))
        .group(&GENERAL_GROUP);

    let token = read_token()?;
    let intents = GatewayIntents::all();
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .map_err(|_| CatError::CannotBuildBot)?;

    client.start().await.map_err(|_| CatError::CannotStartBot)?;

    Ok(())
}
