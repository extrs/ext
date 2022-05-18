use anyhow::Result;
use clap::Subcommand;

use self::completion::CompletionCommand;

mod completion;

/// Manage extrs itself.
#[derive(Debug, Subcommand)]
pub enum SelfCommand {
    Completion(CompletionCommand),
}

impl SelfCommand {
    /// Runs the command.
    pub(crate) async fn run(&self) -> Result<()> {
        match self {
            SelfCommand::Completion(cmd) => cmd.run().await,
        }
    }
}
