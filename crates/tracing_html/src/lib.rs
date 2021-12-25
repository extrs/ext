use anyhow::{Context as _, Error};
use chrono::{NaiveDateTime, Utc};
use serde::Serialize;
use std::{
    collections::HashMap,
    env,
    fs::{self, File},
    io::{BufWriter, Write},
    path::PathBuf,
    sync::Mutex,
};
use tracing::{
    span::{Attributes, Record},
    Id, Subscriber,
};
use tracing_subscriber::{layer::Context, registry::LookupSpan, Layer};

struct HtmlLayer {
    output_path: PathBuf,

    js_path: PathBuf,
    wr: BufWriter<File>,
    data: Mutex<TraceData>,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
struct TraceData {
    span_decls: HashMap<u64, SpanDecl, ahash::RandomState>,
    /// The root span
    root: SpanTraceData,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
struct SpanDecl {
    attrs: HashMap<&'static str, String, ahash::RandomState>,
}

/// Events of a span.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct SpanTraceData {
    #[serde(skip)]
    is_closed: bool,

    events: Vec<Event>,

    spans: Vec<(u64, SpanTraceData)>,

    time: NaiveDateTime,
}

impl Default for SpanTraceData {
    fn default() -> Self {
        Self {
            time: Utc::now().naive_local(),
            is_closed: Default::default(),
            events: Default::default(),
            spans: Default::default(),
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
enum Event {
    #[serde(rename = "span")]
    Span(SpanTraceData),
}

impl Drop for HtmlLayer {
    fn drop(&mut self) {
        let data = serde_json::to_string(&self.data);

        if let Ok(data) = data {
            write!(
                self.wr,
                r#"
<html>
    <head>
        <script id="trace-data" type="text/trace-data">
        {data}
        </script>
    </head>
    <body>
        <div id="root"></div>
        <script src="{js_path}"></script>
    </body>
</html>"#,
                js_path = self.js_path.display(),
                data = data
            )
            .expect("failed to write event to output file");
        }
    }
}

impl SpanTraceData {
    fn with<F, Ret>(&mut self, parent: Option<&Id>, op: F) -> Ret
    where
        F: FnOnce(&mut SpanTraceData) -> Ret,
    {
        // Recursive
        fn find<'a>(from: &'a mut SpanTraceData, id: &Id) -> Option<&'a mut SpanTraceData> {
            for (child_id, span) in &mut from.spans {
                if span.is_closed {
                    continue;
                }

                if id.into_u64() == *child_id {
                    return Some(span);
                }

                if let Some(span) = find(span, id) {
                    return Some(span);
                }
            }
            None
        }

        if let Some(parent) = parent {
            if let Some(v) = find(self, parent) {
                op(v)
            } else {
                unreachable!("{:?} is not a child of {:?}", parent, self)
            }
        } else {
            op(self)
        }
    }

    fn add_span(&mut self, parent: Option<&Id>, id: &Id) {
        self.with(parent, |s| {
            s.spans.push((id.into_u64(), SpanTraceData::default()));
        });
    }

    fn add_record(&mut self, parent: Option<&Id>, span: &Id, values: &Record) {
        self.with(parent, |s| {});
    }

    fn add_event(&mut self, parent: Option<&Id>, event: &tracing::Event) {
        self.with(parent, |s| {});
    }

    fn enter_span(&mut self, parent: Option<&Id>, id: &Id) {
        self.with(parent, |s| {});
    }

    fn exit_span(&mut self, parent: Option<&Id>, id: &Id) {
        self.with(parent, |s| {});
    }

    fn close_span(&mut self, parent: Option<&Id>, id: Id) {
        self.with(parent, |s| {});
    }

    fn change_id(&mut self, parent: Option<&Id>, old: &Id, new: &Id) {
        self.with(parent, |s| {});
    }
}

impl<S> Layer<S> for HtmlLayer
where
    S: Subscriber,
{
    fn on_new_span(&self, attrs: &Attributes<'_>, id: &Id, ctx: Context<'_, S>) {
        let mut w = self.data.lock().unwrap();

        {
            let mut v = w.span_decls.entry(id.into_u64()).or_default();

            // TODO: Attributes
            // TODO: Metadata
        }
        w.root.add_span(ctx.current_span().id(), id);
    }

    fn on_record(&self, span: &Id, values: &Record<'_>, ctx: Context<'_, S>) {
        self.data
            .lock()
            .unwrap()
            .root
            .add_record(ctx.current_span().id(), span, values);
    }

    fn on_event(&self, event: &tracing::Event<'_>, ctx: Context<'_, S>) {
        self.data
            .lock()
            .unwrap()
            .root
            .add_event(ctx.current_span().id(), event);
    }

    fn on_enter(&self, id: &Id, ctx: Context<'_, S>) {
        self.data
            .lock()
            .unwrap()
            .root
            .enter_span(ctx.current_span().id(), id);
    }

    fn on_exit(&self, id: &Id, ctx: Context<'_, S>) {
        self.data
            .lock()
            .unwrap()
            .root
            .exit_span(ctx.current_span().id(), id);
    }

    fn on_close(&self, id: Id, ctx: Context<'_, S>) {
        self.data
            .lock()
            .unwrap()
            .root
            .close_span(ctx.current_span().id(), id);
    }

    fn on_id_change(&self, old: &Id, new: &Id, ctx: Context<'_, S>) {
        self.data
            .lock()
            .unwrap()
            .root
            .change_id(ctx.current_span().id(), old, new);
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

    let wr = BufWriter::new(file);

    Ok(HtmlLayer {
        output_path: output,
        js_path,
        wr,
        data: Default::default(),
    })
}
