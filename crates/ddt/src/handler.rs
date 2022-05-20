use std::{path::PathBuf, sync::Arc};

use anyhow::{anyhow, Result};
use tokio::task::yield_now;

/// Handles one `ddt.yml` file.
#[derive(Debug)]
pub struct FileHandler {
    ddt_file: Arc<PathBuf>,

    event_sender: tokio::sync::mpsc::UnboundedSender<FileHandlerEvent>,
}

#[derive(Debug)]
pub(crate) enum FileHandlerEvent {
    Kill,
    /// File change event
    FileChange(Arc<notify::DebouncedEvent>),
}

impl FileHandler {
    pub(crate) async fn start(ddt_file: Arc<PathBuf>) -> Result<Arc<Self>> {
        let (event_sender, mut event_receiver) = tokio::sync::mpsc::unbounded_channel();

        let server = Arc::new(Self {
            ddt_file,
            event_sender,
        });

        tokio::task::spawn({
            let server = server.clone();
            async move {
                while let Some(event) = event_receiver.recv().await {
                    server.handle_event(event).await?;
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

    pub(crate) async fn send(&self, event: FileHandlerEvent) -> Result<()> {
        self.event_sender.send(event)?;

        yield_now().await;

        Ok(())
    }

    async fn handle_event(self: &Arc<Self>, event: FileHandlerEvent) -> Result<()> {
        Ok(())
    }
}
