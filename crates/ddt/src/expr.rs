use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged, rename_all = "camelCase")]
pub enum Expr {
    /// ```yml  
    /// if:
    ///     - cond a && cond b
    /// ```
    And(Box<Expr>, Box<Expr>),

    /// ```yml  
    /// if:
    ///     - cond a || cond b
    /// ```
    Or(Box<Expr>, Box<Expr>),
}
