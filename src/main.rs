use clap::Parser;

use self::cmd::Command;

mod cmd;

#[derive(Debug, Parser)]
struct CmdArgs {
    #[clap(subcommand)]
    cmd: Command,
}

fn main() {
    println!("Hello, world!");
}
