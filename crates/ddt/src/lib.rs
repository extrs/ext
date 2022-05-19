use std::{
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
};

use anyhow::{Context, Result};
use handler::FileHandler;
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use rustc_hash::FxHashMap;

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

    watcher: Option<RecommendedWatcher>,

    watch_receiver: std::sync::mpsc::Receiver<notify::DebouncedEvent>,
}

impl Server {
    pub fn new(root_dir: &Path) -> Result<Self> {
        let (watch_sender, watch_receiver) = std::sync::mpsc::channel();

        let root_dir = root_dir
            .canonicalize()
            .map(Arc::new)
            .context("failed to canonicalize root dir")?;

        // TODO: Find all `ddt.yml` files in the root directory.
        let handlers = Default::default();

        // TODO: Allow disabling watch
        let mut watcher = notify::watcher(watch_sender, Duration::from_secs(1))?;

        watcher.watch(&**root_dir, RecursiveMode::Recursive)?;

        Ok(Self {
            root_dir,
            handlers,
            watcher: Some(watcher),
            watch_receiver,
        })
    }

    pub async fn run(&self) -> Result<()> {
        Ok(())
    }
}
