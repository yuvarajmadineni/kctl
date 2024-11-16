use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
pub struct CreateTopicCommand {
    pub topic_name: String,

    #[clap(long, default_value = "1")]
    pub partition: i32,

    #[clap(long, default_value = "1")]
    pub replication_factor: i32,
}

#[derive(Parser, Debug)]
pub struct CreateConsumerGroupCommand {
    pub name: String,
    pub topics: Vec<String>,
}

#[derive(Debug, Subcommand)]
pub enum CreateCommands {
    Topic(CreateTopicCommand),
    ConsumerGroup(CreateConsumerGroupCommand),
}

#[derive(Debug, Subcommand)]
pub enum CreateConsumerGroupCommands {
    Topic(CreateConsumerGroupCommand),
}

#[derive(Debug, Args)]
pub struct CreateCommand {
    #[command(subcommand)]
    pub create_cmd: CreateCommands,
}
