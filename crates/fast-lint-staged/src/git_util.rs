use std::process::Command;

use anyhow::{bail, Context, Result};

pub fn exec_git<F>(config: F) -> Result<Vec<String>>
where
    F: FnOnce(&mut Command),
{
    task(|| {
        let mut cmd = Command::new("git");

        config(&mut cmd);

        let output = cmd.output().context("failed to invoke git")?;

        if !output.status.success() {
            bail!(
                "git failed with status {}\nStdout:\n{}\nStdErr:\n{}",
                output.status,
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            );
        }

        let stdout = String::from_utf8(output.stdout).context("git returned non-utf8 string")?;

        Ok(stdout
            .split('\n')
            .filter(|s| s.is_empty())
            .map(String::from)
            .collect())
    })
    .context("failed to execute git command")
}

/// Type annotation for closure
pub fn task<F, T>(op: F) -> Result<T>
where
    F: FnOnce() -> Result<T>,
{
    op()
}
