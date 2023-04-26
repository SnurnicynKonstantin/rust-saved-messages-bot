use serde::Serialize;

use crate::api::error::Error;
use crate::models::enums::ResponseStatus;
use crate::Test;


#[derive(Serialize)]
pub struct TestResponse {
    pub status: ResponseStatus,
    pub data: Option<Vec<Test>>,
}

impl From<Result<Vec<Test>, Error>> for TestResponse {
    fn from(r: Result<Vec<Test>, Error>) -> Self {
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