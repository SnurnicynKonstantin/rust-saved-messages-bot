pub use self::account::*;
use http::StatusCode;

mod account;

#[derive(thiserror::Error, Debug)]
pub enum ControllersError {
    #[error("Invalid requests: `{0}`")]
    WrongInput(String),
}

impl ControllersError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            ControllersError::WrongInput(_) => StatusCode::BAD_REQUEST,
        }
    }
}
