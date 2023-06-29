mod error;
mod events;

use error::CatResult;
use events::Events;
use serenity::{
    prelude::{GatewayIntents, TypeMapKey},
    Client,
};
use sqlx::{mysql::MySqlPoolOptions, MySqlPool};
use std::{env::var, sync::Arc};

struct Database;

impl TypeMapKey for Database {
    type Value = Arc<MySqlPool>;
}

#[tokio::main]
async fn main() -> CatResult<()> {
    let token = var("TOKEN").expect("Enviroment variable TOKEN must be set");
    let db_url = var("DATABASE_URL").expect("Enviroment variable DATABASE_URL must be set");

    env_logger::init();

    let intents = GatewayIntents::all();

    let mut client = Client::builder(token, intents)
        .event_handler(Events)
        .await?;

    {
        let pool = MySqlPoolOptions::new()
            .max_connections(5)
            .connect(db_url.as_str())
            .await?;

        let mut data = client.data.write().await;

        data.insert::<Database>(Arc::new(pool));
    }

    client.start().await?;

    Ok(())
}
