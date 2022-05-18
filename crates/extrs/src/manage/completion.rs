use std::io;

use anyhow::Result;
use clap::{Args, IntoApp};
use clap_complete::{generate, Generator, Shell};

use crate::AppArgs;

/// Generates completion scripts for the shell.
#[derive(Debug, Args)]
pub struct CompletionCommand {
    /// Generate a SHELL completion script and print to stdout
    #[clap(arg_enum, value_name = "SHELL")]
    pub shell: Shell,
}

impl CompletionCommand {
    pub(super) async fn run(&self) -> Result<()> {
        print_completions(self.shell);

        Ok(())
    }
}

fn print_completions<G: Generator>(gen: G) {
    let mut cmd = AppArgs::command();

    let bin_name = cmd.get_name().to_string();

    generate(gen, &mut cmd, bin_name, &mut io::stdout());
}
