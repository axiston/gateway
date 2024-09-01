#[cfg(feature = "support-https")]
use std::path::PathBuf;

use crate::Args;

/// App [`server`] configuration.
///
/// [`server`]: crate::server
#[derive(Debug, Clone)]
#[must_use = "configs do nothing unless you use them"]
pub struct ServerConfig {
    pub port: u16,
    #[cfg(feature = "support-https")]
    pub redirect: u16,
    #[cfg(feature = "support-https")]
    pub cert: PathBuf,
    #[cfg(feature = "support-https")]
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

impl From<Args> for ServerConfig {
    fn from(args: Args) -> Self {
        Self {
            port: args.port,
            #[cfg(feature = "support-https")]
            redirect: args.redirect,
            #[cfg(feature = "support-https")]
            cert: args.keys.join("./cert.pem"),
            #[cfg(feature = "support-https")]
            key: args.keys.join("./key.pem"),
        }
    }
}

/// [`ServerConfig`] builder.
#[derive(Debug, Default, Clone)]
#[must_use = "configs do nothing unless you use them"]
pub struct ServerBuilder {
    pub port: Option<u16>,
    #[cfg(feature = "support-https")]
    pub redirect: Option<u16>,
    #[cfg(feature = "support-https")]
    pub cert: Option<PathBuf>,
    #[cfg(feature = "support-https")]
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
            #[cfg(feature = "support-https")]
            redirect: self.port.unwrap_or(3001),
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
