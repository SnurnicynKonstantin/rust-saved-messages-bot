use serde::{Deserialize, Serialize};

mod kafka_client;

#[derive(Debug, Deserialize, Clone)]
pub struct KafkaClientConfig {
    pub kafka_host: String,
    pub feedback_topic: String,
}

pub struct KafkaClient {
    client: kafka::producer::Producer,
    feedback_topic: String,
}