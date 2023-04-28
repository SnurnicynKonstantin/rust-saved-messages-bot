use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct Account {
    pub id: i64,
    pub name: String,
    pub surname: String,
    pub user_id: i64
}