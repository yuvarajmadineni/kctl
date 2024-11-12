use rdkafka::admin::{AdminOptions, NewTopic};

use crate::admin_kafka;

pub async fn create_topic(topic: NewTopic<'_>) {
    let admin_client = admin_kafka::create_admin_client();
    let created_topic = admin_client
        .create_topics(&[topic], &AdminOptions::default())
        .await
        .expect("Topic creation has failed");

    match &created_topic[0] {
        Ok(topic) => {
            println!("Topic {} has been created successfully", topic);
        }
        Err(e) => {
            eprintln!("Error while creating the topic {}", e.1);
        }
    }
}
