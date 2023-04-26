use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Test {
    pub id: i64,
    pub name: String,
}