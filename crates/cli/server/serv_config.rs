#[cfg(feature = "support-https")]
use std::path::PathBuf;

/// App [`server`] configuration.
///
/// [`server`]: crate::server
#[must_use = "configs do nothing unless you use them"]
#[derive(Debug, Clone)]
pub struct ServerConfig {
    /// Port exposed by the primary server.
    pub port: u16,
    // Shutdown duration in seconds.
    pub shutdown: u64,

    /// Port exposed by the secondary (redirection) server.
    #[cfg(feature = "support-https")]
    #[cfg_attr(docsrs, doc(cfg(feature = "support-https")))]
    pub redirect: u16,

    /// `./cert.pem` file location.
    #[cfg(feature = "support-https")]
    #[cfg_attr(docsrs, doc(cfg(feature = "support-https")))]
    pub cert: PathBuf,

    /// `./key.pem` file location.
    #[cfg(feature = "support-https")]
    #[cfg_attr(docsrs, doc(cfg(feature = "support-https")))]
    pub key: PathBuf,
}

impl ServerConfig {
    /// Returns a new [`ServerConfig`].
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns a new [`ServerBuilder`].
    #[inline]
    pub fn builder() -> ServerBuilder {
        ServerBuilder::new()
    }
}

impl Default for ServerConfig {
    #[inline]
    fn default() -> Self {
        Self::builder().build()
    }
}

/// [`ServerConfig`] builder.
#[must_use = "configs do nothing unless you use them"]
#[derive(Debug, Default, Clone)]
pub struct ServerBuilder {
    pub port: Option<u16>,
    pub shutdown: Option<u64>,

    #[cfg(feature = "support-https")]
    #[cfg_attr(docsrs, doc(cfg(feature = "support-https")))]
    pub redirect: Option<u16>,

    #[cfg(feature = "support-https")]
    #[cfg_attr(docsrs, doc(cfg(feature = "support-https")))]
    pub cert: Option<PathBuf>,

    #[cfg(feature = "support-https")]
    #[cfg_attr(docsrs, doc(cfg(feature = "support-https")))]
    pub key: Option<PathBuf>,
}

impl ServerBuilder {
    /// Returns a new [`ServerBuilder`].
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns a new [`ServerConfig`].
    pub fn build(self) -> ServerConfig {
        ServerConfig {
            port: self.port.unwrap_or(3000),
            shutdown: self.shutdown.unwrap_or(10),
            #[cfg(feature = "support-https")]
            redirect: self.redirect.unwrap_or(3001),
            #[cfg(feature = "support-https")]
            cert: self.cert.unwrap_or(PathBuf::from("./cert.pem")),
            #[cfg(feature = "support-https")]
            key: self.key.unwrap_or(PathBuf::from("./key.pem")),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::server::{ServerBuilder, ServerConfig};

    #[test]
    fn config_from_default() -> anyhow::Result<()> {
        let _ = ServerConfig::default();
        Ok(())
    }

    #[test]
    fn config_from_builder() -> anyhow::Result<()> {
        let _ = ServerBuilder::new().build();
        Ok(())
    }
}
