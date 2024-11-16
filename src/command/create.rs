use rdkafka::admin::{AdminOptions, NewTopic};
use rdkafka::consumer::{Consumer, StreamConsumer};

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

// TODO: need to check why the creation is not happening
pub async fn create_consumer_group(group_id: String, topics: &[&str]) {
    let consumer: StreamConsumer = admin_kafka::create_config()
        .set("group.id", group_id)
        .create()
        .expect("Consumer creation failed");

    let res = consumer.subscribe(topics);

    if let Err(e) = res {
        println!("Error while subscribing to topics: {:?}", e);
        return;
    }

    match consumer.recv().await {
        Ok(msg) => {
            println!("Consumer joined the group {:?}", msg)
        }
        Err(e) => {
            println!("Error while consumer is joining the group {:?}", e);
            return;
        }
    }

    println!("Consumer group created and consuming messages.");
}
