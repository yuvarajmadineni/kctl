use clap::{Args, Subcommand};
#[derive(Clone, Debug, Args)]
pub struct ListCommand {
    #[command(subcommand)]
    pub list_topic_cmd: ListTopicCommands,
}

#[derive(Debug, Subcommand, Clone)]
pub enum ListTopicCommands {
    Topics,
}
