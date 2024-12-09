//! TODO.
//!

mod client_builder;
mod task_registry;

use deadpool::managed::Pool;

pub use crate::client::client_builder::RuntimeClientBuilder;
use crate::runtime::RuntimeManager;
use crate::RuntimeResult;

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
    pub fn register(&self) -> RuntimeResult<()> {
        // self.inner.manager()
        Ok(())
    }

    /// Returns a new [`RuntimeClientBuilder`].
    #[inline]
    pub fn builder() -> RuntimeClientBuilder {
        RuntimeClientBuilder::new()
    }
}

#[cfg(test)]
mod test {
    use crate::{RuntimeResult, RuntimeClient};

    #[test]
    fn build_from_default() -> RuntimeResult<()> {
        let _ = RuntimeClient::builder().build();
        Ok(())
    }
}
