use std::path::Path;

use anyhow::{Context, Result};
use cargo_metadata::{MetadataCommand, PackageId};

use super::wrap_task;

pub fn cargo_workspace_members(from_dir: &Path) -> Result<Vec<PackageId>> {
    wrap_task(|| {
        let md = MetadataCommand::new()
            .no_deps()
            .current_dir(&from_dir)
            .exec()
            .context("cargo metadata failed")?;

        Ok(md.workspace_members)
    })
    .with_context(|| {
        format!(
            "failed to get workspace members from {}",
            from_dir.display()
        )
    })
}
