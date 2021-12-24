use anyhow::{Context, Error};
use std::{
    env,
    fs::{self, File},
    io::{self, BufWriter, Write},
    path::PathBuf,
    sync::Mutex,
};
use tracing::Subscriber;
use tracing_subscriber::{registry::LookupSpan, Layer};

struct HtmlWriter {
    js_path: PathBuf,
    wr: BufWriter<File>,
}

impl io::Write for HtmlWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.wr.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.wr.flush()
    }
}

impl Drop for HtmlWriter {
    fn drop(&mut self) {
        write!(
            self.wr,
            r#"</script>
        </head>
        <body>
            <div id="root"></div>
            <script src="{js_path}"></script>
        </body>
    </html>"#,
            js_path = self.js_path.display()
        )
        .expect("failed to write tail");
    }
}

/// Create a new `Layer` that will write the log messages to a html file.
///
/// `to` is expected to be a path to a html file. (and you should exclude it
/// from vcs)
///
///
/// # Example
///
/// While testing, you can print to the console at a same time.
/// just call `.with()` after `.finish()`.
///
/// ```no_run
/// use tracing_html::html_layer;
/// use tracing_subscriber::prelude::*;
///
/// let logger = tracing_subscriber::FmtSubscriber::builder()
///     .with_test_writer()
///     .pretty()
///     .finish()
///     .with(html_layer("simple1.html".into()).unwrap());
/// ```
pub fn html_layer<S>(output: PathBuf) -> Result<impl Layer<S>, Error>
where
    S: Subscriber + for<'a> LookupSpan<'a>,
{
    let file = fs::OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(&output)
        .unwrap();
    let js_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
        .join("../../apps/tracing-html-viewer/dist/main.js");
    let js_path = js_path.canonicalize().context("failed to canonicalize")?;

    let mut wr = BufWriter::new(file);

    write!(
        wr,
        r#"
    <html>
        <head>
            <script id="trace-data" type="text/trace-data">
    "#,
    )?;

    let writer = HtmlWriter { js_path, wr };

    Ok(tracing_subscriber::fmt::layer()
        .json()
        .with_writer(Mutex::new(writer)))
}
