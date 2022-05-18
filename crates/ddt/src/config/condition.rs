use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged, rename_all = "camelCase")]
pub enum Condition {
    ///
    /// ```yml
    /// 
    /// if:
    ///   - cond a
    ///   - cond b
    /// ```
    Or(Vec<Condition>),

    /// ```yml  
    /// if:
    ///     - cond a && cond b
    /// ```
    And(Box<Condition>, Box<Condition>),
}
