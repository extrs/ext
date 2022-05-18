use std::time::Duration;

use anyhow::{Context, Result};
use clap::Parser;
use ext_common::init_logger;
use tokio::process::Command;

/// Run command with a timeout.
///
/// Usage: `ext-timeout --timeout 5s -- cargo build --release`
#[derive(Debug, Parser)]
#[clap(name = "ext-timeout")]
pub struct AppArgs {
    /// The number of milliseconds to wait before terminating, or text like `1s`
    /// or `1m`.
    ///
    /// --timeout 1s is identical to --timeout 1000
    #[clap(long)]
    pub timeout: String,

    pub command: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let _logger = init_logger();

    let args = AppArgs::parse();

    let timeout = args
        .timeout
        .parse::<u64>()
        .map(Duration::from_millis)
        .or_else(|_| {
            humantime::parse_duration(&args.timeout).context("failed to parse timeout duration")
        })?;

    let mut cmd = Command::new(&args.command[0]);
    cmd.kill_on_drop(true);

    for arg in args.command.iter().skip(1) {
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
