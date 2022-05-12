use std::{
    path::Path,
    process::{Command, Stdio},
};

use anyhow::{bail, Context, Result};
use cargo_metadata::{MetadataCommand, PackageId};
use cargo_platform::Cfg;

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

pub fn query_rustc_cfg() -> Result<(String, Vec<Cfg>)> {
    wrap_task(|| {
        let mut c = Command::new("cargo");
        c.arg("rustc");
        c.arg("-Z")
            .arg("unstable-options")
            .arg("--print")
            .arg("cfg");

        c.stderr(Stdio::inherit());

        let output = c.output().context("cargo rustc failed")?;

        if !output.status.success() {
            bail!("rustc failed");
        }

        let cfg_lines =
            String::from_utf8(output.stdout).context("cargo rustc emitted non-utf8 string")?;

        let cfg_list = cfg_lines
            .lines()
            .map(|s| s.parse::<Cfg>().context("failed parse cfg"))
            .collect::<Result<Vec<_>>>()?;

        Ok(("TODO".into(), cfg_list))
    })
    .context("failed to query rustc for cfg and platform")
}
