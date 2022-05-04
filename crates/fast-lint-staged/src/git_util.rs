use std::{
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::{bail, Context, Result};

use crate::util::task;

pub fn exec_git<F>(config: F) -> Result<String>
where
    F: FnOnce(&mut Command),
{
    task(|| {
        let mut cmd = Command::new("git");

        cmd.arg("-c").arg("submodule.recurse=false");

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

        Ok(stdout)
    })
    .context("failed to execute git command")
}

pub fn get_staged_files(cwd: Option<&Path>) -> Result<Vec<PathBuf>> {
    task(|| {
        let lines = exec_git(|cmd| {
            cmd.arg("diff")
                .arg("--staged")
                .arg("--diff-filter=ACMR")
                .arg("--name-only")
                .arg("-z");
        })
        .context("failed to execute git diff")?;

        if lines.is_empty() {
            return Ok(Default::default());
        }

        parse_git_z_output(&lines)
            .into_iter()
            .map(PathBuf::from)
            .map(|file| file.canonicalize().context("failed to normalize path"))
            .collect::<Result<_>>()
    })
    .context("failed to get staged files")
}

/// Return array of strings split from the output of `git <something> -z`. With
/// `-z`, git prints `fileA\u0000fileB\u0000fileC\u0000` so we need to remove
/// the last occurrence of `\u0000` before splitting
fn parse_git_z_output(input: &str) -> Vec<String> {
    if input.is_empty() {
        return vec![];
    }

    input
        .strip_suffix("\u{0000}")
        .unwrap()
        .split("\u{0000}")
        .map(String::from)
        .collect()
}
