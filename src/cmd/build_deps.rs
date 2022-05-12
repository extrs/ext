use std::{
    collections::{HashMap, HashSet},
    env::current_dir,
    sync::Arc,
};

use anyhow::{anyhow, bail, Context, Result};
use cargo_metadata::{Dependency, MetadataCommand, Package, PackageId, Version};
use cargo_platform::Cfg;
use clap::Args;

use crate::util::cargo::query_rustc_cfg;

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

        let (cur_target, cur_cfgs) = query_rustc_cfg()?;

        let mut finder = DepsFinder {
            workspace_members: metadata.workspace_members,
            cur_target,
            cur_cfgs,
            pkg_by_name: Default::default(),
            done: Default::default(),
            deps: Default::default(),
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

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct DepPkgConfig {
    features: Vec<String>,

    use_default_feature: bool,
}

impl DepPkgConfig {
    fn merge(&mut self, other: Self) {
        self.features.extend_from_slice(&other.features);
        self.features.sort();
        self.features.dedup();

        self.use_default_feature |= other.use_default_feature;
    }
}

#[derive(Debug)]
struct DepsFinder {
    cur_target: String,
    cur_cfgs: Vec<Cfg>,

    workspace_members: Vec<PackageId>,
    pkg_by_name: HashMap<String, Vec<Arc<Package>>>,

    done: HashSet<PackageId>,

    deps: HashMap<PackageId, DepPkgConfig>,
}

impl DepsFinder {
    /// If cargo is not going to build a package it's not in the list.
    fn pkg_from_dep(&self, dep: &Dependency) -> Option<Arc<Package>> {
        let pkgs = self.pkg_by_name.get(&dep.name)?;

        for pkg in pkgs.iter() {
            if dep.req.matches(&pkg.version) {
                return Some(pkg.clone());
            }
        }

        None
    }

    fn check(&mut self, pkg: &Package) -> Result<()> {
        if !self.done.insert(pkg.id.clone()) {
            return Ok(());
        }

        if self.workspace_members.contains(&pkg.id) {
            self.include_pkg(pkg)?;
        }

        Ok(())
    }

    fn include_pkg(&mut self, pkg: &Package) -> Result<()> {
        for dep in &pkg.dependencies {
            let pkg = self.pkg_from_dep(dep);
            let pkg = match pkg {
                Some(v) => v,
                None => continue,
            };

            if let Some(config) = self.check_dep(pkg.clone(), dep)? {
                self.check(&pkg)?;

                if !self.workspace_members.contains(&pkg.id) {
                    self.deps.entry(pkg.id.clone()).or_default().merge(config);
                }
            }
        }

        Ok(())
    }

    /// Returns [Some] if `dep` should be built.
    fn check_dep(&mut self, pkg: Arc<Package>, dep: &Dependency) -> Result<Option<DepPkgConfig>> {
        // TODO: Optional
        // TODO: build/dev deps

        // Check if we are going to include this package.
        match &dep.target {
            Some(target) => {
                if !target.matches(&self.cur_target, &self.cur_cfgs) {
                    // We are not interested in this package.
                    return Ok(None);
                }
            }
            None => {
                // This is simple, unconditional dependency.
            }
        }

        Ok(Some(DepPkgConfig {
            features: dep.features.clone(),
            use_default_feature: dep.uses_default_features,
        }))
    }
}
