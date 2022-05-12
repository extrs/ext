use std::{
    collections::{HashMap, HashSet},
    env::current_dir,
    sync::Arc,
};

use anyhow::{anyhow, bail, Context, Result};
use cargo_metadata::{Dependency, MetadataCommand, Package, PackageId, Version};
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
        let metadata = MetadataCommand::new()
            .exec()
            .context("cargo metadata failed")?;

        let mut finder = DepsFinder {
            workspace_members: metadata.workspace_members,
            ..Default::default()
        };

        let mut all_pkgs = vec![];

        for pkg in metadata.packages.into_iter().map(Arc::new) {
            all_pkgs.push(pkg.clone());

            finder
                .pkg_by_name
                .entry(pkg.name.clone())
                .or_default()
                .push(pkg.clone());
        }

        for pkg in &all_pkgs {
            finder.check(pkg)?;
        }

        dbg!(&finder.deps);

        Ok(())
    }
}

/// Build config for a package.

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct DepPkgConfig {
    features: Vec<String>,
}

impl DepPkgConfig {
    fn merge(&mut self, other: Self) {
        self.features.extend_from_slice(&other.features);
        self.features.sort();
        self.features.dedup();
    }
}

#[derive(Debug, Default)]
struct DepsFinder {
    workspace_members: Vec<PackageId>,
    pkg_by_name: HashMap<String, Vec<Arc<Package>>>,

    deps: HashMap<PackageId, DepPkgConfig>,
}

impl DepsFinder {
    fn pkg_from_dep(&self, dep: &Dependency) -> Result<Arc<Package>> {
        let pkgs = self
            .pkg_by_name
            .get(&dep.name)
            .ok_or_else(|| anyhow!("failed to find package {}", dep.name))?;

        for pkg in pkgs.iter() {
            if dep.req.matches(&pkg.version) {
                return Ok(pkg.clone());
            }
        }

        bail!("failed to find matching version of {}", dep.name)
    }

    fn check(&mut self, pkg: &Package) -> Result<()> {
        if self.workspace_members.contains(&pkg.id) {
            self.include_pkg(pkg)?;
        }

        Ok(())
    }

    fn include_pkg(&mut self, pkg: &Package) -> Result<()> {
        for dep in &pkg.dependencies {
            let pkg = self.pkg_from_dep(dep)?;

            if let Some(config) = self.check_dep(dep)? {}
        }

        Ok(())
    }

    /// Returns [Some] if `dep` should be built.
    fn check_dep(&mut self, dep: &Dependency) -> Result<Option<DepPkgConfig>> {}
}
