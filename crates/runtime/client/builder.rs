use crate::runtime::RuntimeManager;
use crate::Client;

/// [`Client`] builder.
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

    /// Builds a new [`Client`].
    pub fn build(self) -> Client {
        Client::new(self.inner.pool())
    }
}
