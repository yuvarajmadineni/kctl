use std::{time::Duration, vec};

use clap::{Parser, Subcommand};
use cli::config::{
    create::{CreateCommand, CreateTopicCommands},
    delete::{DeleteCommand, DeleteTopicCommands},
    list::{ListCommand, ListTopicCommands},
};
use rdkafka::{
    admin::{AdminOptions, NewTopic, TopicReplication},
    consumer::{BaseConsumer, Consumer, DefaultConsumerContext},
};

mod admin_kafka;
mod cli;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Create(CreateCommand),
    List(ListCommand),
    Delete(DeleteCommand),
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    match args.cmd {
        Commands::Create(topic) => match topic.creat_topic_cmd {
            CreateTopicCommands::Topic(topic) => {
                let name = topic.topic_name;
                let partition = topic.partition;
                let replication_factor = TopicReplication::Fixed(topic.replication_factor);
                let new_topic = NewTopic {
                    name: &name,
                    num_partitions: partition,
                    replication: replication_factor,
                    config: vec![],
                };
                create_topics(new_topic).await;
            }
        },
        Commands::List(topics) => match topics.list_topic_cmd {
            ListTopicCommands::Topics => {
                let topics = list_topics().await;
                println!("{:?}", topics)
            }
        },
        Commands::Delete(topic) => match topic.delete_topic_cmd {
            DeleteTopicCommands::Topic(topic) => delete_topic(topic.topic_name).await,
        },
    }
}

async fn create_topics(topic: NewTopic<'_>) {
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

async fn list_topics() -> Vec<String> {
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

async fn delete_topic(topic_name: String) {
    let admin_client = admin_kafka::create_admin_client();
    let res = admin_client
        .delete_topics(&[&topic_name], &AdminOptions::default())
        .await;

    match res {
        Ok(inner_res) => match &inner_res[0] {
            Ok(data) => {
                println!("Topic {} deleted successfully", data);
            }

            Err(e) => {
                println!("Error while deleting topic {:?}", e)
            }
        },
        Err(e) => {
            println!("Error while deleting the topic {:?}", e)
        }
    }
}
