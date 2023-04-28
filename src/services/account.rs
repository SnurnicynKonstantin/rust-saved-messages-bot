use http::StatusCode;
use crate::{SqlxClient, Account};
use crate::api::Error;

pub struct AccountService {
    sqlx_client: SqlxClient,
}

impl AccountService {
    pub fn new(
        sqlx_client: SqlxClient
    ) -> Self {
        Self {
            sqlx_client
        }
    }

    pub async fn get_accounts(
        &self,
    ) -> Result<Vec<Account>, Error> {
        let accounts = self
            .sqlx_client
            .get_accounts()
            .await?;

        Ok(accounts)
    }

    pub async fn save_account(
        &self,
        account: Account
    ) -> Result<Account, Error> {
        let account = self
            .sqlx_client
            .save_account(account)
            .await?;

        Ok(account)
    }

    pub async fn get_account_by_id(
        &self,
        id: i64
    ) -> Result<Account, Error> {
        let account = self
            .sqlx_client
            .get_account_by_id(id)
            .await?;

        Ok(account)
    }

}

#[derive(thiserror::Error, Debug)]//Move to enum
pub enum ServiceError {
    #[error("Invalid requests: `{0}`")]
    WrongInput(String),
    #[error("Service unavailable")]
    ServiceUnavailable,
}

impl ServiceError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            ServiceError::WrongInput(_) => {
                StatusCode::BAD_REQUEST
            }
            ServiceError::ServiceUnavailable => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }
}
