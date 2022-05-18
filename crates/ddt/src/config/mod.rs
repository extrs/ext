use serde::Deserialize;

/// One config file
#[derive(Debug, Clone, Deserialize)]
pub struct DdtConfigFile {
    rules: Vec<DdtRule>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DdtRule {}
