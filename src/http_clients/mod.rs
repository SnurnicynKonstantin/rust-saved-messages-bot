use serde::{Deserialize, Serialize};

mod http_client;

#[derive(Debug, Deserialize, Clone)]
pub struct HttpClientConfig {
    pub api_url: String,
    pub api_key: String,
}

#[derive(Clone)]
pub struct HttpClient {
    client: reqwest::Client,
    api_url: String,
    api_key: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub struct ConvertResponse {
    pub success: Option<bool>,
    pub query: Option<Query>,
    pub info: Option<Info>,
    pub date: Option<String>,
    pub result: Option<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Query {
    pub from: String,
    pub to: String,
    pub amount: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Info {
    pub timestamp: String,
    pub rate: String,
}