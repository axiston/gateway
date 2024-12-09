use crate::client::RuntimeConn;

/// [`RuntimeConn`] builder.
#[derive(Debug, Default, Clone)]
#[must_use = "builders do nothing unless you use them"]
pub struct RuntimeConnBuilder {}

impl RuntimeConnBuilder {
    /// Returns a new [`RuntimeConnBuilder`].
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Builds a new [`RuntimeConn`].
    pub fn build(self) -> RuntimeConn {
        // TODO: Build endpoint here.
        todo!()
    }
}
