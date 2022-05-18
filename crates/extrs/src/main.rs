use anyhow::Result;
use clap::{Parser, Subcommand};
use ext_common::init_logger;
use ext_timeout::TimeoutCommand;
use extrs_self::SelfCommand;

#[derive(Debug, Parser)]
#[clap(name = "extrs")]
struct AppArgs {
    #[clap(subcommand)]
    cmd: ExtCommand,
}

#[derive(Debug, Subcommand)]
enum ExtCommand {
    Timeout(TimeoutCommand),
    #[clap(subcommand)]
    X(SelfCommand),
}

#[tokio::main]
async fn main() -> Result<()> {
    let _logger = init_logger();

    let args = AppArgs::parse();

    match args.cmd {
        ExtCommand::Timeout(cmd) => cmd.run().await,
        ExtCommand::X(cmd) => cmd.run().await,
    }
}
