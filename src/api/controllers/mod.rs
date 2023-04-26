pub use self::test::*;
use http::StatusCode;

mod test;

#[derive(thiserror::Error, Debug)]
pub enum ControllersError {
    #[error("Invalid request: `{0}`")]
    WrongInput(String),
}

impl ControllersError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            ControllersError::WrongInput(_) => StatusCode::BAD_REQUEST,
        }
    }
}
