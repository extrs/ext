use anyhow::Result;
use clap::{Parser, Subcommand};
use ext_common::init_logger;
use ext_timeout::TimeoutCommand;

use self::manage::ManageCommand;

mod manage;

#[derive(Debug, Parser)]
#[clap(name = "extrs")]
struct AppArgs {
    #[clap(subcommand)]
    cmd: ExtCommand,
}

#[derive(Debug, Subcommand)]
enum ExtCommand {
    Timeout(TimeoutCommand),
    #[clap(name = "x", subcommand)]
    ManageSelf(ManageCommand),
}

#[tokio::main]
async fn main() -> Result<()> {
    let _logger = init_logger();

    let args = AppArgs::parse();

    match args.cmd {
        ExtCommand::Timeout(cmd) => cmd.run().await,
        ExtCommand::ManageSelf(cmd) => cmd.run().await,
    }
}
