use rdkafka::consumer::{BaseConsumer, Consumer, DefaultConsumerContext};
use std::time::Duration;

use crate::admin_kafka;

pub async fn list_topics() -> Vec<String> {
    let consumer: BaseConsumer<DefaultConsumerContext> = admin_kafka::create_config()
        .create()
        .expect("Consumer creation failed");
    let metadata = consumer
        .fetch_metadata(None, Some(Duration::from_secs(1)))
        .map_err(|e| e.to_string());

    match metadata {
        Ok(meta) => {
            let topics: Vec<String> = meta
                .topics()
                .iter()
                .map(|topic| topic.name().to_string())
                .collect();
            topics
        }
        Err(e) => {
            println!("Error while fetching metadata {:?}", e);
            return Vec::new();
        }
    }
}
