use anyhow::{Result};
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

pub async fn pool(database_path: String) -> Result<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_path)
        .await.expect("Unable connect to Postgres");
    Ok(pool)
}