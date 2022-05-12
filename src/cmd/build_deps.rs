use std::{collections::HashSet, env::current_dir};

use anyhow::{Context, Result};
use cargo_metadata::{MetadataCommand, Package, PackageId, Version};
use clap::Args;

use crate::util::cargo::cargo_workspace_members;

/// Used to build only dependencies, excluding workspace members.
///
/// This can be used for e.g. shared caching across workspace while testing each
/// crate in a parallel.
#[derive(Debug, Args)]
pub struct BuildDepsCommand {}

impl BuildDepsCommand {
    pub fn run(self) -> Result<()> {
        let dir = current_dir().context("failed to get current directory")?;
        let workspace_members = cargo_workspace_members(&dir)?;

        let mut finder = DepsFinder {
            workspace_members,
            deps: Default::default(),
        };

        let metadata = MetadataCommand::new()
            .exec()
            .context("cargo metadata failed")?;

        for pkg in &metadata.packages {
            finder.check(pkg)?;
        }

        dbg!(&finder.deps);

        Ok(())
    }
}

#[derive(Debug)]
struct DepsFinder {
    workspace_members: Vec<PackageId>,

    deps: Vec<(String, Version)>,
}

impl DepsFinder {
    fn check(&mut self, pkg: &Package) -> Result<()> {
        if self.workspace_members.contains(&pkg.id) {
            self.include_pkg(pkg)?;
        }

        Ok(())
    }

    fn include_pkg(&mut self, pkg: &Package) -> Result<()> {
        for dep in &pkg.dependencies {}

        Ok(())
    }
}
