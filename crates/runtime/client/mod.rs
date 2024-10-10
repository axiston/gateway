//! TODO.
//!

mod client_builder;
mod task_registry;

use deadpool::managed::Pool;

pub use crate::client::client_builder::ClientBuilder;
use crate::runtime::RuntimeManager;
use crate::Result;

/// TODO.
#[derive(Debug, Clone)]
pub struct RuntimeClient {
    inner: Pool<RuntimeManager>,
}

impl RuntimeClient {
    /// Returns a new [`RuntimeClient`].
    #[inline]
    fn new(inner: Pool<RuntimeManager>) -> Self {
        Self { inner }
    }

    /// Dynamically registers TODO.
    pub fn register(&self) -> Result<()> {
        // self.inner.manager()
        Ok(())
    }

    /// Returns a new [`ClientBuilder`].
    #[inline]
    pub fn builder() -> ClientBuilder {
        ClientBuilder::new()
    }
}

#[cfg(test)]
mod test {
    use crate::{Result, RuntimeClient};

    #[test]
    fn build_from_default() -> Result<()> {
        let _ = RuntimeClient::builder().build();
        Ok(())
    }
}
