use anyhow::Error;
use tracing::{debug, error, info, trace, warn};
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

    #[tracing::instrument]
    fn deeply_nested(call: usize) {
        if call == 0 {
            tracing::info!("Zero");
        }

        trace!("Called");
        debug!("Called");
        info!(kind = "perf", "Called");
        warn!("Called");
        error!("Called");

        if call > 0 {
            deeply_nested(call - 1);
        }
    }

    tracing::error!("Hello, world!");
    call(10);
    call(5);
    call(2);

    deeply_nested(110);

    Ok(())
}
