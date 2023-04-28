use anyhow::Result;
use crate::{SqlxClient, Account};

impl SqlxClient {
    pub async fn get_accounts(&self) -> Result<Vec<Account>> {
        sqlx::query_as!(Account,
                "SELECT * FROM account",
            )
            .fetch_all(&self.pool)
            .await
            .map_err(From::from)
    }

    pub async fn save_account(&self, account: Account) -> Result<Account> {
        sqlx::query_as!(Account,
                "INSERT INTO account (name, surname, user_id) VALUES ($1, $2, $3) RETURNING id, name, surname, user_id",
                account.name,
                account.surname,
                account.user_id,
            )
            .fetch_one(&self.pool)
            .await
            .map_err(From::from)
    }

    pub async fn get_account_by_id(&self, id: i64) -> Result<Account> {
        sqlx::query_as!(Account,
                "SELECT * FROM account WHERE id = $1",
                id
            )
            .fetch_one(&self.pool)
            .await
            .map_err(From::from)
    }
}