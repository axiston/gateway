use serde::{Deserialize, Serialize};

/// Configures `RuntimeManager` for one or more runtimes.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[must_use = "configs do nothing unless you use them"]
pub struct RuntimeManagerConfig {
    /// Method of how a connection is recycled.
    ///
    /// See [`RecyclingMethod`].
    pub recycling_method: RecyclingMethod,
}

impl RuntimeManagerConfig {
    /// Returns a new [`RuntimeManagerConfig`].
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Overrides the value of [`RuntimeManagerConfig`]`::recycling_method`.
    pub fn with_recycling_method(mut self, recycling_method: RecyclingMethod) -> Self {
        self.recycling_method = recycling_method;
        self
    }
}

/// TODO.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub enum RecyclingMethod {
    /// TODO.
    #[default]
    Fast,
    /// TODO.
    Verified,
}

impl RecyclingMethod {
    /// Returns a new [`RecyclingMethod`].
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }
}

#[cfg(test)]
mod test {
    use crate::manager::RuntimeManagerConfig;
    use crate::Result;

    #[test]
    fn default_settings() -> Result<()> {
        let _ = RuntimeManagerConfig::new();
        Ok(())
    }
}
