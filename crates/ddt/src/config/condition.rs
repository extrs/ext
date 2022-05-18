use serde::Deserialize;

use crate::expr::Expr;

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
    Multi(Vec<Expr>),
    Single(Expr),
}
