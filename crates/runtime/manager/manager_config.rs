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

/// Possible methods of how a connection is recycled.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub enum RecyclingMethod {
    /// Only check for open event bus when recycling existing connections
    /// Unless you have special needs this is a safe choice.
    #[default]
    Fast,
    /// In addition to checking for open event bus a test query is executed.
    ///
    /// This is slower, but guarantees that the database connection is ready to be used.
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
    fn build_default_settings() -> Result<()> {
        let _ = RuntimeManagerConfig::new();
        Ok(())
    }
}
