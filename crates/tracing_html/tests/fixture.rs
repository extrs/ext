use anyhow::Error;
use tracing_html::html_layer;
use tracing_subscriber::prelude::*;

fn init(name: &str) -> Result<tracing::dispatcher::DefaultGuard, Error> {
    let logger = tracing_subscriber::FmtSubscriber::builder()
        .without_time()
        .with_target(false)
        .with_ansi(true)
        .with_test_writer()
        .pretty()
        .finish()
        .with(html_layer(name.into())?);

    Ok(tracing::subscriber::set_default(logger))
}

#[test]
fn simple_1() -> Result<(), Error> {
    let _guard = init("simple1.html")?;

    tracing::info!("Hello, world!");

    Ok(())
}

#[test]
fn multiple() -> Result<(), Error> {
    let _guard = init("multiple.html")?;

    tracing::info!("Hello, world!");
    tracing::error!("Hello, world!");

    Ok(())
}

#[test]
fn instrument() -> Result<(), Error> {
    let _guard = init("instrument.html")?;

    #[tracing::instrument]
    fn call() {
        tracing::info!("Hello, world!");
    }

    tracing::error!("Hello, world!");
    call();

    Ok(())
}

#[test]
fn instrument_nested() -> Result<(), Error> {
    let _guard = init("instrument_nested.html")?;

    #[tracing::instrument]
    fn call(arg: usize) {
        for _ in 0..5 {
            tracing::info!("Hello, world!");
            call2();
        }
    }

    #[tracing::instrument]
    fn call2() {
        tracing::warn!("Hello, world!");
    }

    tracing::error!("Hello, world!");
    call(10);
    call(5);
    call(2);

    Ok(())
}
