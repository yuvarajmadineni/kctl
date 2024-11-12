use clap::{Args, Subcommand};

#[derive(Debug, Subcommand)]
pub enum DeleteTopicCommands {
    Topic(DeleteTopicCommand),
}

#[derive(Debug, Args)]
pub struct DeleteCommand {
    #[command(subcommand)]
    pub delete_topic_cmd: DeleteTopicCommands,
}

#[derive(Debug, Args)]
pub struct DeleteTopicCommand {
    pub topic_name: String,
}
