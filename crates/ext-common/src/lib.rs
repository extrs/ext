pub fn init_logger() -> tracing::subscriber::DefaultGuard {
    let logger = tracing_subscriber::fmt()
        .with_ansi(true)
        .with_level(true)
        .with_thread_ids(false)
        .with_thread_names(false)
        .with_writer(std::io::stderr)
        .finish();

    tracing::subscriber::set_default(logger)
}
