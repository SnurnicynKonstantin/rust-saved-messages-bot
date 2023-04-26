use anyhow::Result;
use crate::{SqlxClient, Test};

impl SqlxClient {
    pub async fn get_tests(
        &self
    ) -> Result<Vec<Test>> {
        sqlx::query_as!(Test,
                "SELECT * FROM test",
            )
            .fetch_all(&self.pool)
            .await
            .map_err(From::from)
    }
}