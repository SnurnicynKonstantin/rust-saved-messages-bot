use serde::Serialize;

use crate::api::error::Error;
use crate::models::enums::ResponseStatus;
use crate::Account;


#[derive(Serialize)]
pub struct AccountResponse {
    pub status: ResponseStatus,
    pub data: Option<Vec<Account>>,
}

impl From<Result<Vec<Account>, Error>> for AccountResponse {
    fn from(r: Result<Vec<Account>, Error>) -> Self {
        match r {
            Ok(data) => Self {
                status: ResponseStatus::Ok,
                data: Some(data),
            },
            Err(_) => Self {
                status: ResponseStatus::Error,
                data: None,
            },
        }
    }
}

impl From<Result<Account, Error>> for AccountResponse {
    fn from(r: Result<Account, Error>) -> Self {
        match r {
            Ok(data) => Self {
                status: ResponseStatus::Ok,
                data: Some(vec![data]),
            },
            Err(_) => Self {
                status: ResponseStatus::Error,
                data: None,
            },
        }
    }
}