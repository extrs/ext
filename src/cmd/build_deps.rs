use anyhow::{Context, Result};
use cargo_metadata::MetadataCommand;
use clap::Args;

/// Used to build only dependencies, excluding workspace members.
///
/// This can be used for e.g. shared caching across workspace while testing each
/// crate in a parallel.
#[derive(Debug, Args)]
pub struct BuildDepsCommand {}

impl BuildDepsCommand {
    pub fn run(self) -> Result<()> {
        let metadata = MetadataCommand::new()
            .exec()
            .context("cargo metadata failed")?;

        Ok(())
    }
}
