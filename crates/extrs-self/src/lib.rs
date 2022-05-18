use anyhow::Result;
use clap::Subcommand;

use self::completion::CompletionCommand;

mod completion;

/// Manages `extrs` itself.
#[derive(Debug, Subcommand)]
pub enum SelfCommand {
    Completion(CompletionCommand),
}

impl SelfCommand {
    /// Runs the command.
    pub async fn run(&self) -> Result<()> {
        match self {
            SelfCommand::Completion(cmd) => cmd.run().await,
        }
    }
}
