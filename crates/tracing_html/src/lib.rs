use anyhow::Error;
use std::{
    fs::{self, File},
    io::{self, BufWriter, Write},
    path::PathBuf,
    sync::Mutex,
};
use tracing::Subscriber;
use tracing_subscriber::{registry::LookupSpan, Layer};

struct HtmlWriter {
    to: PathBuf,
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

/// Create a new `Layer` that will write the log messages to a html file.
///
/// `to` is expected to be a path to a html file. (and you should exclude it
/// from vcs)
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

    let mut wr = BufWriter::new(file);

    write!(wr, "<html>")?;

    let writer = HtmlWriter { to: output, wr };

    Ok(tracing_subscriber::fmt::layer()
        .json()
        .with_writer(Mutex::new(writer)))
}
