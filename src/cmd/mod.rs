use clap::Subcommand;

use self::build_deps::BuildDepsCommand;

mod build_deps;

#[derive(Debug, Subcommand)]
pub enum Command {
    BuildDeps(BuildDepsCommand),
}
