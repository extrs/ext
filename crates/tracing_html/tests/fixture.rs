use anyhow::Error;
use tracing_html::html_layer;
use tracing_subscriber::prelude::*;

fn init() -> Result<tracing::dispatcher::DefaultGuard, Error> {
    let logger = tracing_subscriber::FmtSubscriber::builder()
        .without_time()
        .with_target(false)
        .with_ansi(true)
        .with_test_writer()
        .pretty()
        .finish()
        .with(html_layer("simple1.html".into())?);

    Ok(tracing::subscriber::set_default(logger))
}

#[test]
fn simple_1() -> Result<(), Error> {
    let _guard = init()?;

    tracing::info!("Hello, world!");

    Ok(())
}
