use anyhow::{Context as _, Error};
use chrono::{NaiveDateTime, Utc};
use serde::Serialize;
use std::{
    collections::HashMap,
    env,
    fmt::Debug,
    fs::{self, File},
    io::{BufWriter, Write},
    path::PathBuf,
    sync::Mutex,
};
use tracing::{
    field::Field,
    span::{Attributes, Record},
    Id, Subscriber,
};
use tracing_subscriber::{layer::Context, registry::LookupSpan, Layer};

type Fields = HashMap<&'static str, String, ahash::RandomState>;

struct HtmlLayer {
    #[allow(dead_code)]
    output_path: PathBuf,

    js_path: PathBuf,
    wr: BufWriter<File>,
    data: Mutex<TraceData>,
}

#[derive(Debug, Serialize)]
struct Metadata {
    level: String,
    name: &'static str,
    target: String,
    module_path: Option<String>,
    file: Option<String>,
    line: Option<u32>,
}

impl From<&'_ tracing::Metadata<'_>> for Metadata {
    fn from(md: &tracing::Metadata) -> Self {
        Metadata {
            level: md.level().to_string(),
            name: md.name(),
            target: md.target().to_string(),
            module_path: md.module_path().map(|p| p.to_string()),
            file: md.file().map(|p| p.to_string()),
            line: md.line(),
        }
    }
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
struct TraceData {
    span_decls: HashMap<u64, SpanDecl, ahash::RandomState>,
    /// The root span
    root: SpanTraceData,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct SpanDecl {
    attrs: Fields,

    metadata: Metadata,
}

/// Events of a span.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct SpanTraceData {
    exited_at: Option<NaiveDateTime>,

    closed_at: Option<NaiveDateTime>,

    entered_at: Option<NaiveDateTime>,

    created_at: NaiveDateTime,

    events: Vec<Event>,

    spans: Vec<(u64, SpanTraceData)>,
}

impl Default for SpanTraceData {
    fn default() -> Self {
        Self {
            created_at: Utc::now().naive_local(),
            exited_at: Default::default(),
            events: Default::default(),
            spans: Default::default(),
            closed_at: Default::default(),
            entered_at: Default::default(),
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct Event {
    fields: Fields,

    metadata: Metadata,
}

impl Drop for HtmlLayer {
    fn drop(&mut self) {
        let data = serde_json::to_string(&self.data);

        if let Ok(data) = data {
            write!(
                self.wr,
                r#"
<!DOCTYPE html>
<html>
    <head>
        <meta charset="utf-8">
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
                // Ignore if the span is already closed
                if span.exited_at.is_some() {
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

    #[allow(unused_variables)]
    fn add_record(&mut self, parent: Option<&Id>, span: &Id, values: &Record) {
        self.with(parent, |s| {});
    }

    fn add_event(&mut self, parent: Option<&Id>, event: &tracing::Event) {
        self.with(parent, |s| {
            let mut fields = Fields::default();
            event.record(&mut |f: &Field, v: &dyn Debug| {
                fields.insert(f.name(), format!("{:?}", v));
            });

            s.events.push(Event {
                fields,
                metadata: event.metadata().into(),
            });
        });
    }

    fn enter_span(&mut self, parent: Option<&Id>, id: &Id) {
        self.with(parent, |s| {
            for (child_id, v) in s.spans.iter_mut() {
                if *child_id == id.into_u64() {
                    v.entered_at = Some(Utc::now().naive_local());
                }
            }
        });
    }

    fn exit_span(&mut self, parent: Option<&Id>, id: &Id) {
        self.with(parent, |s| {
            for (child_id, v) in s.spans.iter_mut() {
                if *child_id == id.into_u64() {
                    v.exited_at = Some(Utc::now().naive_local());
                }
            }
        });
    }

    fn close_span(&mut self, parent: Option<&Id>, id: Id) {
        self.with(parent, |s| {
            for (child_id, v) in s.spans.iter_mut() {
                if *child_id == id.into_u64() {
                    v.closed_at = Some(Utc::now().naive_local());
                }
            }
        });
    }

    fn change_id(&mut self, parent: Option<&Id>, old: &Id, new: &Id) {
        self.with(parent, |s| {
            for (id, _) in s.spans.iter_mut() {
                if *id == old.into_u64() {
                    *id = new.into_u64();
                }
            }
        });
    }
}

impl<S> Layer<S> for HtmlLayer
where
    S: Subscriber,
{
    fn on_new_span(&self, attrs: &Attributes<'_>, id: &Id, ctx: Context<'_, S>) {
        let mut w = self.data.lock().unwrap();

        {
            let decl = w
                .span_decls
                .entry(id.into_u64())
                .or_insert_with(|| SpanDecl {
                    attrs: Default::default(),
                    metadata: attrs.metadata().into(),
                });

            attrs.record(&mut |f: &Field, v: &dyn Debug| {
                decl.attrs.insert(f.name(), format!("{:?}", v));
            });
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
