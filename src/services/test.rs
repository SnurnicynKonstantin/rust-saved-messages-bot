use http::StatusCode;
use crate::{SqlxClient, Test};
use crate::api::Error;

// #[derive(Clone)]
pub struct TestService {
    sqlx_client: SqlxClient,
}

impl TestService {
    pub fn new(
        sqlx_client: SqlxClient
    ) -> Self {
        Self {
            sqlx_client
        }
    }

    pub async fn get_tests(
        &self,
    ) -> Result<Vec<Test>, Error> {
        let tests = self
            .sqlx_client
            .get_tests()
            .await?;

        Ok(tests)
    }

//     Implement get by id

}

#[derive(thiserror::Error, Debug)]
pub enum TestServiceError {
    #[error("Invalid request: `{0}`")]
    WrongInput(String),
    #[error("Service unavailable")]
    ServiceUnavailable,
}

impl TestServiceError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            TestServiceError::WrongInput(_) => {
                StatusCode::BAD_REQUEST
            }
            TestServiceError::ServiceUnavailable => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }
}
