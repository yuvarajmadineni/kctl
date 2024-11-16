use clap::{Args, Subcommand};
#[derive(Clone, Debug, Args)]
pub struct ListCommand {
    #[command(subcommand)]
    pub list_cmd: ListCommands,
}

#[derive(Debug, Subcommand, Clone)]
pub enum ListCommands {
    Topics,
    ConsumerGroups
}

