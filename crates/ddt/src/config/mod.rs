use serde::Deserialize;

use self::condition::Condition;

pub mod condition;

/// One config file
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigFile {
    pub rules: Vec<Rule>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rule {
    #[serde(default)]
    pub name: Option<String>,

    #[serde(rename = "if")]
    pub if_: Condition,

    pub steps: Vec<RuleStep>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuleStep {
    #[serde(default)]
    pub name: Option<String>,
}
