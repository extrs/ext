use serde::Deserialize;

use self::condition::Condition;

pub mod condition;

/// One config file
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigFile {
    pub(crate) rules: Vec<Rule>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rule {
    pub(crate) actions: Vec<RuleAction>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuleAction {
    #[serde(rename = "if")]
    pub(crate) if_: Condition,
}
