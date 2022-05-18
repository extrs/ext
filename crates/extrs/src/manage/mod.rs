use anyhow::Result;
use clap::Subcommand;

use self::completion::CompletionCommand;

mod completion;

/// Manage extrs itself.
#[derive(Debug, Subcommand)]
#[clap(name = "x")]
pub enum ManageCommand {
    Completion(CompletionCommand),
}

impl ManageCommand {
    /// Runs the command.
    pub(crate) async fn run(&self) -> Result<()> {
        match self {
            ManageCommand::Completion(cmd) => cmd.run().await,
        }
    }
}
