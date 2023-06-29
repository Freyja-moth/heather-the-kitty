mod error;
mod events;

use error::CatResult;
use events::Events;
use serenity::{
    prelude::{GatewayIntents, TypeMapKey},
    Client,
};
use sqlx::{mysql::MySqlPoolOptions, MySqlPool};
use std::sync::Arc;

struct Database;

impl TypeMapKey for Database {
    type Value = Arc<MySqlPool>;
}

#[tokio::main]
async fn main() -> CatResult<()> {
    env_logger::init();

    let intents = GatewayIntents::all();
    let token = env!("TOKEN");

    let mut client = Client::builder(token, intents)
        .event_handler(Events)
        .await?;

    {
        let pool = MySqlPoolOptions::new()
            .max_connections(5)
            .connect("mariadb://freyja-moth:Transbian but gayer@freyja-desktop/heather")
            .await?;

        let mut data = client.data.write().await;

        data.insert::<Database>(Arc::new(pool));
    }

    client.start().await?;

    Ok(())
}
