use anyhow::{Result, anyhow};
use crate::http_clients::{ConvertResponse, HttpClientConfig, HttpClient};

const API_KEY: &str = "apikey";

impl HttpClient {
    pub fn new(config: HttpClientConfig) -> Self {
        let api_url = config.api_url;
        let api_key = config.api_key;
        let client = reqwest::Client::builder()
            .build()
            .expect("Failed to create http client");
        Self { client, api_url, api_key, }
    }

    pub async fn get_course(&self, from: String, to: String, amount: i16) -> Result<String> {
        self.client
            .get(format!("{}/?to={to}&from={from}&amount={amount}", self.api_url))
            .header(API_KEY, self.api_key.as_str())
            .send()
            .await?
            .json::<ConvertResponse>()
            .await? // unwraps std::Result<_, reqwest::Error>
            .result
            .ok_or(anyhow!("No result, there is an an error in request.")) // wraps into std::Result<_, anyhow::Error>
    }
}