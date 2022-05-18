use anyhow::Result;
use clap::Args;

/// Generates completion scripts for the shell.
#[derive(Debug, Args)]
pub struct CompletionCommand {}

impl CompletionCommand {
    pub(super) async fn run(&self) -> Result<()> {
        Ok(())
    }
}
