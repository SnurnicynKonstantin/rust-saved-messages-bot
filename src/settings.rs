use anyhow::{Result};
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use serde::{Deserialize};
use crate::HttpClientConfig;
use crate::KafkaClientConfig;

pub async fn pool(database_path: String) -> Result<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_path)
        .await.expect("Unable connect to Postgres");
    Ok(pool)
}

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub kafka_client: KafkaClientConfig,
    pub http_client: HttpClientConfig,
}

impl Config {
    pub fn new(config_name: String) -> Config {
        let mut settings = config::Config::new();

        if std::path::Path::new(&config_name).exists() {
            settings.merge(config::File::with_name(format!("./{config_name}").as_str())).unwrap();
        } else {
            panic!("Can't find file with name like: {config_name}");
        }

        let conf = settings
            .try_into::<Config>()
            .unwrap_or_else(|e| panic!("Error parsing config: {}", e));

        conf
    }
}