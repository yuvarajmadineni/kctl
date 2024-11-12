use clap::{Parser, Subcommand};
use cli::config::{
    create::{CreateCommand, CreateTopicCommands},
    delete::{DeleteCommand, DeleteTopicCommands},
    list::{ListCommand, ListTopicCommands},
};
use command::create::create_topic;
use command::delete::delete_topic;
use command::list::list_topics;
use rdkafka::admin::{NewTopic, TopicReplication};

mod admin_kafka;
mod cli;
mod command;

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
                create_topic(new_topic).await;
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
