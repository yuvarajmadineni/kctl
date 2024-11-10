use std::{time::Duration, vec};

use clap::{Args, Parser, Subcommand};
use rdkafka::{
    admin::{AdminOptions, NewTopic, TopicReplication},
    consumer::{BaseConsumer, Consumer, DefaultConsumerContext},
};

mod admin_kafka;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    Create(Createtopic),
    List(Listtopic),
}

#[derive(Clone, Debug, Args)]
struct Createtopic {
    #[command(subcommand)]
    cmd: CreateTopicCommands,
}

#[derive(Clone, Debug, Args)]
struct Listtopic {
    #[command(subcommand)]
    cmd: ListTopicCommands,
}

#[derive(Debug, Subcommand, Clone)]
enum CreateTopicCommands {
    Topic { topic: String },
}

#[derive(Debug, Subcommand, Clone)]
enum ListTopicCommands {
    Topics,
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    match args.cmd {
        Commands::Create(topic) => match topic.cmd {
            CreateTopicCommands::Topic { topic } => {
                let new_topic = NewTopic {
                    name: &topic,
                    num_partitions: 1,
                    replication: TopicReplication::Fixed(1),
                    config: vec![],
                };
                create_topics(new_topic).await;
                println!("Topic {} has been created successfully", topic);
            }
        },
        Commands::List(topics) => match topics.cmd {
            ListTopicCommands::Topics => {
                let topics = list_topics().await;
                println!("{:?}", topics)
            }
        },
    }
}

async fn create_topics(topic: NewTopic<'_>) {
    let admin_client = admin_kafka::create_admin_client();
    let _ = admin_client
        .create_topics(&[topic], &AdminOptions::default())
        .await
        .expect("Topic creation has failed");
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
