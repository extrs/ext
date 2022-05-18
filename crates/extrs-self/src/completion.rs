use anyhow::Result;
use clap::Args;

#[derive(Debug, Args)]
pub struct CompletionCommand {}

impl CompletionCommand {
    pub(super) async fn run(&self) -> Result<()> {
        Ok(())
    }
}
