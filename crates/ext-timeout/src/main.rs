use anyhow::Result;
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

    println!("Hello, world!");

    Ok(())
}
