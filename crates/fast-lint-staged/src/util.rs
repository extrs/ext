use anyhow::Result;

/// Type annotation for closure
pub fn task<F, T>(op: F) -> Result<T>
where
    F: FnOnce() -> Result<T>,
{
    op()
}
