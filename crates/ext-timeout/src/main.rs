use std::{
    process::{self, Command},
    sync::mpsc,
    thread,
    time::Duration,
};

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

    let (sender, receiver) = mpsc::channel();

    let timer_sender = sender.clone();

    let _t = thread::spawn(move || {
        let res = (|| {
            let mut cmd = Command::new(&args.command[0]);
            for (i, arg) in args.command.iter().enumerate() {
                if i == 0 {
                    continue;
                }
                cmd.arg(arg);
            }

            let status = cmd.status().context("failed to run command")?;

            Ok(status)
        })();

        let _ = sender.send(res);
    });

    let _timer = thread::spawn(move || {
        thread::sleep(timeout);
        let _ = timer_sender.send(Err(anyhow!("timed out")));
    });

    let status = receiver.recv().unwrap()?;

    process::exit(status.code().unwrap_or(1));
}
