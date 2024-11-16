use rdkafka::consumer::{BaseConsumer, Consumer, DefaultConsumerContext};
use std::time::Duration;

use crate::admin_kafka;

pub async fn list_topics() {
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
            println!("{:?}", topics)
        }
        Err(e) => {
            println!("Error while fetching metadata {:?}", e);
        }
    }
}

// TODO: fix it later
pub async fn list_consumer_groups() {
    let consumer: BaseConsumer<DefaultConsumerContext> = admin_kafka::create_config()
        .create()
        .expect("Consumer creation failed");

    let res = consumer.fetch_group_list(None, Some(Duration::from_secs(1)));

    match res {
        Ok(data) => {
            let groups = data.groups().iter().map(|info| info.name());
            println!("{:?}", groups)
        }
        Err(e) => {
            println!("Error while fetching groups {:?}", e)
        }
    }
}
