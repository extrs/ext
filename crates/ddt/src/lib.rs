pub mod config;
pub mod config_file;
pub mod expr;

/// Manages ddt tasks in a directory and subdirectories.
#[derive(Debug)]
pub struct DirServer {
    subdirectories: Vec<DirServer>,
}
