use crate::runtime::RuntimeManager;
use crate::RuntimeClient;

/// [`RuntimeClient`] builder.
#[derive(Debug, Default)]
pub struct ClientBuilder {
    inner: RuntimeManager,
}

impl ClientBuilder {
    /// Returns a new [`ClientBuilder`].
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Builds a new [`RuntimeClient`].
    pub fn build(self) -> RuntimeClient {
        RuntimeClient::new(self.inner.pool())
    }
}
