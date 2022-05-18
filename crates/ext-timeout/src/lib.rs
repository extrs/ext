use std::time::Duration;

use anyhow::{Context, Result};
use clap::Parser;
use tokio::process::Command;

/// Run command with a timeout.
///
/// Usage: `ext-timeout --timeout 5s -- cargo build --release`
#[derive(Debug, Parser)]
#[clap(name = "timeout")]
pub struct TimeoutCommand {
    /// The number of milliseconds to wait before terminating, or text like `1s`
    /// or `1m`.
    ///
    /// --timeout 1s is identical to --timeout 1000
    #[clap(long)]
    pub timeout: String,

    pub command: Vec<String>,
}

impl TimeoutCommand {
    pub async fn run(self) -> Result<()> {
        let timeout = self
            .timeout
            .parse::<u64>()
            .map(Duration::from_millis)
            .or_else(|_| {
                humantime::parse_duration(&self.timeout).context("failed to parse timeout duration")
            })?;

        let mut cmd = Command::new(&self.command[0]);
        cmd.kill_on_drop(true);

        for arg in self.command.iter().skip(1) {
            cmd.arg(arg);
        }

        let status = tokio::time::timeout(timeout, async move {
            cmd.status().await.context("failed to run command")
        })
        .await;

        match status {
            Ok(status) => match status {
                Ok(status) => std::process::exit(status.code().unwrap_or(1)),
                Err(err) => {
                    // Child process failed to start
                    eprintln!("{}", err);
                    std::process::exit(2);
                }
            },
            Err(..) => {
                // Timed out
                eprintln!("Timeout");
                std::process::exit(1);
            }
        }
    }
}
