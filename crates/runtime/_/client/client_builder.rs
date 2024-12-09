use crate::runtime::RuntimeManager;
use crate::{RuntimeResult, RuntimeClient};

/// [`RuntimeClient`] builder.
#[derive(Debug, Default)]
pub struct RuntimeClientBuilder {
    inner: RuntimeManager,
}

impl RuntimeClientBuilder {
    /// Returns a new [`RuntimeClientBuilder`].
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Builds a new [`RuntimeClient`].
    pub fn build(self) -> RuntimeResult<RuntimeClient> {
        let pool = self.inner.pool();
        Ok(RuntimeClient::new(pool))
    }
}

#[cfg(test)]
mod test {
    use crate::{RuntimeResult, RuntimeClientBuilder};

    fn instance() -> RuntimeResult<()> {
        let builder = RuntimeClientBuilder::new();
        let _client = builder.build()?;
    }
}
