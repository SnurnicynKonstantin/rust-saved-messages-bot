use std::sync::Arc;
use crate::services::AccountService;

mod controllers;
pub mod routers;
mod responses;
mod requests;
mod error;

pub use self::error::*;

type Result<T, E = Error> = std::result::Result<T, E>;

pub struct ApiContext {
    account_service: Arc<AccountService>,
}