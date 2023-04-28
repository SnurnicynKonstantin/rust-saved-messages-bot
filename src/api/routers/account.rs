use axum::{
    routing::{get, post},
    Router,
};
use crate::api::controllers;

pub fn router() -> Router {
    Router::new()
        .route("/", get(controllers::get_accounts))
        .route("/id/:id", get(controllers::get_account_by_id))
        .route("/", post(controllers::save_account))
}
