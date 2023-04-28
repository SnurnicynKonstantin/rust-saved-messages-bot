use axum::{Extension, Router};
use std::sync::Arc;
use crate::api::ApiContext;
use crate::AccountService;

mod account;

const API_PREFIX: &str = "/api";

pub fn router(
    account_service: Arc<AccountService>,
) -> Router {
    Router::new()
        .nest(API_PREFIX, api_router(account_service))
}

fn api_router(
    account_service: Arc<AccountService>,
) -> Router {
    Router::new()
        .nest("/account", account::router())
        .layer(Extension(Arc::new(ApiContext {
            account_service,
        })))
}