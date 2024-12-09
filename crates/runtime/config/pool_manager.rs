use deadpool::managed::{Manager, Metrics, RecycleResult};
use serde::{Deserialize, Serialize};

use crate::{RuntimeConn, RuntimeConnBuilder, RuntimeConnError};

#[derive(Debug)]
pub struct RuntimeManager {
    addr: (),
}

impl RuntimeManager {
    /// Returns a new [`RuntimeManager`].
    #[inline]
    pub fn new(addr: ()) -> Self {
        Self::new_with_config(addr, RuntimeManagerConfig::new())
    }

    /// Returns a new [`RuntimeManager`] with a custom configuration.
    pub fn new_with_config(addr: (), config: RuntimeManagerConfig) -> Self {
        todo!()
    }
}

impl Manager for RuntimeManager {
    type Type = RuntimeConn;
    type Error = RuntimeConnError;

    async fn create(&self) -> Result<Self::Type, Self::Error> {
        let builder = RuntimeConnBuilder::new();
        todo!()
    }

    async fn recycle(
        &self,
        conn: &mut Self::Type,
        metrics: &Metrics,
    ) -> RecycleResult<Self::Error> {
        todo!()
    }
}

/// Configures `RuntimeManager` for one or more runtimes.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[must_use = "configs do nothing unless you use them"]
pub struct RuntimeManagerConfig {}

impl RuntimeManagerConfig {
    /// Returns a new [`RuntimeManagerConfig`].
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }
}

#[cfg(test)]
mod test {
    use crate::config::RuntimeManagerConfig;
    use crate::RuntimeResult;

    #[test]
    fn default_settings() -> RuntimeResult<()> {
        let _ = RuntimeManagerConfig::new();
        Ok(())
    }
}
