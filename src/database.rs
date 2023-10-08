use crate::prelude::*;

#[derive(Clone)]
pub struct Database {
    pub database: MySqlPool,
}
impl TypeMapKey for Database {
    type Value = Arc<Self>;
}
impl From<MySqlPool> for Database {
    fn from(database: MySqlPool) -> Self {
        Self { database }
    }
}
impl Database {
    pub async fn connect_to_database(database_url: String) -> KittyResult<Arc<Self>> {
        Ok(MySqlPool::connect(&database_url)
            .await
            .map(Self::from)
            .map(Arc::from)
            .map_err(DatabaseError::CannotConnectToDatabase)?)
    }
    pub async fn retrieve_database(ctx: &Context) -> Option<Arc<Self>> {
        let eyes = ctx.data.read().await;
        eyes.get::<Self>().cloned()
    }
    pub fn inner(&self) -> &MySqlPool {
        &self.database
    }
}
