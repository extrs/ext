use std::{path::PathBuf, sync::Arc};

/// Handles one `ddt.yml` file.
#[derive(Debug)]
pub struct FileHandler {
    ddt_file: Arc<PathBuf>,
}
