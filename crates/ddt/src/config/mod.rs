use serde::Deserialize;

use self::condition::Condition;

pub mod condition;

/// One config file
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigFile {
    #[serde(default)]
    pub actions: Vec<Action>,

    #[serde(default)]
    pub rules: Vec<Rule>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Action {
    pub id: String,

    #[serde(default)]
    pub stdin: bool,

    #[serde(default)]
    pub name: Option<String>,

    pub steps: Vec<RuleStep>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rule {
    #[serde(default)]
    pub id: Option<String>,

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
    pub id: Option<String>,

    #[serde(default)]
    pub name: Option<String>,
}
