use anyhow::Result;
use clap::Parser;

use self::cmd::Command;

mod cmd;
mod util;

#[derive(Debug, Parser)]
struct CmdArgs {
    #[clap(subcommand)]
    cmd: Command,
}

fn main() -> Result<()> {
    let args = CmdArgs::parse();

    match args.cmd {
        Command::BuildDeps(cmd) => cmd.run()?,
    }

    Ok(())
}
