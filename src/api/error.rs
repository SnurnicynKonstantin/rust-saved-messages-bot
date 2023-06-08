use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use tracing::log;

use crate::api::controllers::ControllersError;
use crate::services::ServiceError;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("an error occurred with the database")]
    Sqlx(#[from] sqlx::Error),

    #[error("an error occurred with serde")]
    Serde(#[from] serde_json::Error),

    #[error("an internal server error occurred")]
    Anyhow(#[from] anyhow::Error),

    #[error(transparent)]
    AccountService(#[from] ServiceError),
    #[error(transparent)]
    Controllers(#[from] ControllersError),
}

impl Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::Sqlx(_)
            | Self::Serde(_)
            | Self::Anyhow(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::AccountService(e) => e.status_code(),
            Error::Controllers(e) => e.status_code(),
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Self::Sqlx(ref e) => {
                // TODO: we probably want to use `tracing` instead
                // so that this gets linked to the HTTP requests by `TraceLayer`.
                log::error!("SQLx error: {:?}", e);
            }

            Self::Serde(ref e) => {
                // TODO: we probably want to use `tracing` instead
                // so that this gets linked to the HTTP requests by `TraceLayer`.
                log::error!("Serde error: {:?}", e);
            }

            Self::Anyhow(ref e) => {
                // TODO: we probably want to use `tracing` instead
                // so that this gets linked to the HTTP requests by `TraceLayer`.
                log::error!("Generic error: {:?}", e);
            }

            // Other errors get mapped normally.
            Error::AccountService(ref e) => {
                log::error!("Account service error: {:?}", e);
            }

            Error::Controllers(ref e) => {
                log::error!("Controllers error: {:?}", e);
            }
        }

        (
            self.status_code(),
            Json(serde_json::json!({"reason":self.to_string()})),
        )
            .into_response()
    }
}
