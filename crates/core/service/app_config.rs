/// App [`state`] configuration.
///
/// [`state`]: crate::service::AppState
#[derive(Debug, Clone)]
#[must_use = "configs do nothing unless you use them"]
pub struct AppConfig {
    /// TODO: Store database conn string as a URL.
    pub database: String,
}

impl AppConfig {
    /// Returns a new [`AppConfig`].
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns a new [`AppBuilder`].
    #[inline]
    pub fn builder() -> AppBuilder {
        AppBuilder::new()
    }
}

impl Default for AppConfig {
    #[inline]
    fn default() -> Self {
        Self::builder().build()
    }
}

/// [`AppConfig`] builder.
#[derive(Debug, Default, Clone)]
#[must_use = "configs do nothing unless you use them"]
pub struct AppBuilder {
    pub database: Option<String>,
}

impl AppBuilder {
    /// Returns a new [`AppBuilder`].
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns a new [`AppConfig`].
    pub fn build(self) -> AppConfig {
        let default_database = "postgresql://usr:pwd@localhost:5432/db".to_owned();
        AppConfig {
            database: self.database.unwrap_or(default_database),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::service::{AppBuilder, AppConfig};

    #[test]
    fn config_from_default() -> anyhow::Result<()> {
        let _ = AppConfig::default();
        Ok(())
    }

    #[test]
    fn config_from_builder() -> anyhow::Result<()> {
        let _ = AppBuilder::new().build();
        Ok(())
    }
}
