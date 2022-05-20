use std::{
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
};

use anyhow::{anyhow, Context, Result};
use handler::{FileHandler, FileHandlerEvent};
use notify::{RecursiveMode, Watcher};
use rustc_hash::FxHashMap;
use tokio::{
    sync::{mpsc::UnboundedSender, RwLock},
    task::{spawn_blocking, yield_now},
};

pub mod config;
mod config_file;
pub mod expr;
mod handler;

/// Manages ddt tasks in a directory.
///
/// - Watches the root directory
pub struct Server {
    root_dir: Arc<PathBuf>,

    event_sender: UnboundedSender<Event>,

    handlers: RwLock<FxHashMap<Arc<PathBuf>, Arc<FileHandler>>>,
}

#[derive(Debug)]
enum Event {
    Kill,
    FileChange(Arc<notify::DebouncedEvent>),
}

impl Server {
    pub async fn run(root_dir: &Path) -> Result<Arc<Self>> {
        let root_dir = root_dir
            .canonicalize()
            .map(Arc::new)
            .context("failed to canonicalize root dir")?;

        // TODO: Find all `ddt.yml` files in the root directory.
        let handlers = Default::default();

        let (event_sender, mut event_receiver) = tokio::sync::mpsc::unbounded_channel();

        let server = Arc::new(Self {
            root_dir: root_dir.clone(),
            event_sender: event_sender.clone(),
            handlers,
        });

        let (term_sender, mut term_receiver) = tokio::sync::oneshot::channel();

        let _ = spawn_blocking({
            let root_dir = root_dir.clone();
            let event_sender = event_sender.clone();

            move || -> Result<_> {
                let (watch_sender, watch_receiver) = std::sync::mpsc::channel();

                // TODO: Allow disabling watch
                let mut watcher = notify::watcher(watch_sender, Duration::from_secs(1))?;

                watcher.watch(&**root_dir, RecursiveMode::Recursive)?;

                loop {
                    if let Ok(event) = watch_receiver.recv_timeout(Duration::from_millis(50)) {
                        event_sender.send(Event::FileChange(Arc::new(event)))?;
                    }

                    if let Ok(..) = term_receiver.try_recv() {
                        return Ok(());
                    }
                }
            }
        });

        let _ = tokio::task::spawn({
            let server = server.clone();

            async move {
                while let Some(event) = event_receiver.recv().await {
                    match &event {
                        Event::Kill => {
                            event_receiver.close();
                            let _ = term_sender.send(());

                            // Kill file handlers
                            server.handle_event(event).await?;

                            return Ok(());
                        }
                        _ => {
                            let _ = tokio::spawn({
                                let server = server.clone();
                                async move {
                                    let res = server.handle_event(event).await;

                                    dbg!(&res);
                                }
                            });

                            yield_now().await;
                        }
                    }
                }

                // type ann
                if false {
                    return Err(anyhow!(""));
                }

                Ok(())
            }
        });

        yield_now().await;

        Ok(server)
    }

    async fn handle_event(self: &Arc<Self>, event: Event) -> Result<()> {
        match event {
            Event::Kill => {
                // Kill all file handlers

                for handler in self.handlers.read().await.values() {
                    handler.send(FileHandlerEvent::Kill).await?;
                }

                Ok(())
            }
            Event::FileChange(e) => {
                // Dispatch event to file handlers

                for handler in self.handlers.read().await.values() {
                    handler
                        .send(FileHandlerEvent::FileChange(e.clone()))
                        .await?;
                }

                // TODO: Check if the file is a `ddt.yml` file.

                Ok(())
            }
        }
    }

    pub async fn kill(&self) -> Result<()> {
        self.event_sender
            .send(Event::Kill)
            .context("failed to kill")?;

        yield_now().await;

        Ok(())
    }
}
