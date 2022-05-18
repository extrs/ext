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

    pub actions: Vec<RuleAction>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuleAction {
    #[serde(default)]
    pub name: Option<String>,
}
