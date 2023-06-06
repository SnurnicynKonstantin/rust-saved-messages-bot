use kafka::producer::{Producer, Record};
use crate::kafka_clients::{KafkaClientConfig, KafkaClient};

impl KafkaClient {
    pub fn new(config: KafkaClientConfig) -> Self {
        let feedback_topic = config.feedback_topic;
        let host = vec![config.kafka_host.to_owned()];
        let mut client = Producer::from_hosts(host)
            .create()
            .unwrap();
        Self { client, feedback_topic, }
    }

    pub async fn send_feedback(&self, message: &String) {
        // &self.client
        //     .send(&Record::from_value(self.feedback_topic.as_str(), message.as_bytes()))
        //     .unwrap();
    }
}