use std::time::Duration;

use anyhow::{anyhow, Context, Result};
use clap::Parser;
use ext_common::init_logger;

/// Run command with a timeout.
#[derive(Debug, Parser)]
#[clap(name = "timeout")]
pub struct AppArgs {
    /// The number of milliseconds to wait before terminating, or text like `1s`
    /// or `1m`.
    ///
    /// --timeout 1s is identical to --timeout 1000
    #[clap(long)]
    pub timeout: String,

    pub command: Vec<String>,
}

fn main() -> Result<()> {
    let _logger = init_logger();

    let args = AppArgs::parse();

    let timeout = args
        .timeout
        .parse::<u64>()
        .map(Duration::from_millis)
        .or_else(|_| {
            humantime::parse_duration(&args.timeout).context("failed to parse timeout duration")
        })?;

    println!("Hello, world!");

    Ok(())
}
