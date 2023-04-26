use axum::{Extension, Router};
use std::sync::Arc;
use crate::api::ApiContext;
use crate::TestService;

mod test;

const API_PREFIX: &str = "/api";

pub fn router(
    test_service: Arc<TestService>,
) -> Router {
    Router::new()
        .nest(API_PREFIX, api_router(test_service))
}

fn api_router(
    test_service: Arc<TestService>,
) -> Router {
    Router::new()
        .nest("/test", test::router())
        .layer(Extension(Arc::new(ApiContext {
            test_service,
        })))
}