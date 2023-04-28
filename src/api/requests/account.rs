use serde::Deserialize;

use crate::Account;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateAccountRequest {
    pub name: String,
    pub surname: String,
    pub user_id: i64,
}

impl From<CreateAccountRequest> for Account {
    fn from(c: CreateAccountRequest) -> Self {
        Account {
            id: 0,
            name: c.name,
            surname: c.surname,
            user_id: c.user_id
        }
    }
}