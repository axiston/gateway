//! TODO.
//!

mod builder;

use deadpool::managed::Pool;

pub use crate::client::builder::ClientBuilder;
use crate::runtime::RuntimeManager;
use crate::Result;

/// TODO.
#[derive(Debug, Clone)]
pub struct Client {
    inner: Pool<RuntimeManager>,
}

impl Client {
    /// Returns a new [`Client`].
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
    use crate::{Client, Result};

    #[test]
    fn build_from_default() -> Result<()> {
        let _ = Client::builder().build();
        Ok(())
    }
}
