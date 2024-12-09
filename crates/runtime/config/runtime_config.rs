use std::time::Duration;

use serde::{Deserialize, Serialize};

/// Configures [`Runtime`] for one or more runtimes.
///
/// [`Runtime`]: crate::Runtime
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[must_use = "configs do nothing unless you use them"]
pub struct RuntimeConfig {
    pub max_conn: Option<usize>,
    pub create_timeout: Option<Duration>,
    pub wait_timeout: Option<Duration>,
    pub recycle_timeout: Option<Duration>,
}

impl RuntimeConfig {
    /// Creates a new [`RuntimeConfig`].
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Overwrites the default value of [`RuntimeConfig`]`::max_conn`.
    pub fn with_max_conn(mut self, max_conn: usize) -> Self {
        self.max_conn = Some(max_conn);
        self
    }

    /// Overwrites the default value of [`RuntimeConfig`]`::create_timeout`.
    pub fn with_create_timeout(mut self, create_timeout: Duration) -> Self {
        self.create_timeout = Some(create_timeout);
        self
    }

    /// Overwrites the default value of [`RuntimeConfig`]`::wait_timeout`.
    pub fn with_wait_timeout(mut self, wait_timeout: Duration) -> Self {
        self.wait_timeout = Some(wait_timeout);
        self
    }

    /// Overwrites the default value of [`RuntimeConfig`]`::recycle_timeout`.
    pub fn with_recycle_timeout(mut self, recycle_timeout: Duration) -> Self {
        self.recycle_timeout = Some(recycle_timeout);
        self
    }
}

#[cfg(test)]
mod test {
    use crate::{RuntimeConfig, RuntimeResult};

    #[test]
    fn default_settings() -> RuntimeResult<()> {
        let _ = RuntimeConfig::new();
        Ok(())
    }
}
