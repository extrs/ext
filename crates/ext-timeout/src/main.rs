use anyhow::Result;
use clap::Parser;
use ext_common::init_logger;

#[derive(Debug, Parser)]
pub struct AppArgs {}

fn main() -> Result<()> {
    let _logger = init_logger();

    let args = AppArgs::parse();

    println!("Hello, world!");

    Ok(())
}
