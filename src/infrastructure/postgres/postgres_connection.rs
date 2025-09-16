use anyhow::Result;
use std::time::Duration;

use diesel_async::{
    AsyncPgConnection,
    pooled_connection::{AsyncDieselConnectionManager, bb8::Pool},
};

pub type PgPoolSquad = Pool<AsyncPgConnection>;

pub async fn establish_connection(database_url: &str) -> Result<PgPoolSquad> {
    let config = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(database_url);
    let pool = Pool::builder()
        .connection_timeout(Duration::from_secs(1))
        .build(config)
        .await;
    Ok(pool?)
}
