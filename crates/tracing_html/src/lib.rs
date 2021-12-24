use std::{io, path::PathBuf, sync::Mutex};
use tracing::Subscriber;
use tracing_subscriber::{registry::LookupSpan, Layer};

struct HtmlWriter {
    to: PathBuf,
    json: Vec<u8>,
}

impl io::Write for HtmlWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.json.extend_from_slice(buf);

        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

/// Create a new `Layer` that will write the log messages to a html file.
///
/// `to` is expected to be a path to a html file. (and you should exclude it
/// from vcs)
pub fn html_layer<S>(output: PathBuf) -> impl Layer<S>
where
    S: Subscriber + for<'a> LookupSpan<'a>,
{
    let writer = HtmlWriter {
        to: output,
        json: Default::default(),
    };

    tracing_subscriber::fmt::layer()
        .json()
        .with_writer(Mutex::new(writer))
}
