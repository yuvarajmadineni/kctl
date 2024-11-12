use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
pub struct CreateTopicCommand {
    pub topic_name: String,

    #[clap(long, default_value = "1")]
    pub partition: i32,

    #[clap(long, default_value = "1")]
    pub replication_factor: i32,
}

#[derive(Debug, Subcommand)]
pub enum CreateTopicCommands {
    Topic(CreateTopicCommand),
}

#[derive(Debug, Args)]
pub struct CreateCommand {
    #[command(subcommand)]
    pub creat_topic_cmd: CreateTopicCommands,
}
