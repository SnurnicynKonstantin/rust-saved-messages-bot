use std::sync::Arc;
use crate::services::TestService;

mod controllers;
pub mod routers;
mod responses;
mod error;

pub use self::error::*;

type Result<T, E = Error> = std::result::Result<T, E>;

pub struct ApiContext {
    test_service: Arc<TestService>,
}