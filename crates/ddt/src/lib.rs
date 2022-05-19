use std::{
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
};

use anyhow::{anyhow, Context, Result};
use handler::FileHandler;
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use rustc_hash::FxHashMap;
use tokio::{task::spawn_blocking, try_join};

pub mod config;
mod config_file;
pub mod expr;
mod handler;

/// Manages ddt tasks in a directory.
///
/// - Watches the root directory
pub struct Server {
    root_dir: Arc<PathBuf>,

    handlers: FxHashMap<Arc<PathBuf>, FileHandler>,
}

#[derive(Debug)]
enum Event {
    FileChange(notify::DebouncedEvent),
}

impl Server {
    pub fn new(root_dir: &Path) -> Result<Self> {
        let root_dir = root_dir
            .canonicalize()
            .map(Arc::new)
            .context("failed to canonicalize root dir")?;

        // TODO: Find all `ddt.yml` files in the root directory.
        let handlers = Default::default();

        Ok(Self { root_dir, handlers })
    }

    pub async fn run(self: Arc<Self>) -> Result<()> {
        let (event_sender, mut event_receiver) = tokio::sync::mpsc::unbounded_channel();

        let watcher_future = spawn_blocking({
            let server = self.clone();
            let event_sender = event_sender.clone();

            move || -> Result<_> {
                let (watch_sender, watch_receiver) = std::sync::mpsc::channel();

                // TODO: Allow disabling watch
                let mut watcher = notify::watcher(watch_sender, Duration::from_secs(1))?;

                watcher.watch(&**server.root_dir, RecursiveMode::Recursive)?;

                while let Ok(event) = watch_receiver.recv() {
                    event_sender.send(Event::FileChange(event))?;
                }

                Ok(())
            }
        });

        let handler_future = tokio::spawn(async move {
            while let Some(event) = event_receiver.recv().await {
                self.handle_event(event).await?;
            }

            // type ann
            if false {
                return Err(anyhow!(""));
            }

            Ok(())
        });

        let (wr, hr) = try_join!(watcher_future, handler_future)?;

        hr?;
        wr?;

        Ok(())
    }

    async fn handle_event(self: &Arc<Self>, event: Event) -> Result<()> {
        Ok(())
    }
}
