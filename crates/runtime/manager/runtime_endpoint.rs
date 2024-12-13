use std::sync::LazyLock;

use tonic::codegen::Bytes;
use tonic::transport::{Endpoint, Uri};

use crate::{Error, Result};

/// Builds and configures `HTTP/2` channels.
///
/// Includes configuration for the manager.
#[derive(Debug, Clone)]
pub struct RuntimeEndpoint {
    pub(crate) endpoint: Endpoint,
    pub(crate) limit: Option<u32>,
    pub(crate) current: u32,
}

impl RuntimeEndpoint {
    /// Returns a new [`RuntimeEndpoint`].
    pub fn new(endpoint: Endpoint) -> Self {
        Self {
            endpoint,
            limit: None,
            current: 0,
        }
    }

    /// Returns a new [`RuntimeEndpoint`].
    pub fn try_new<A: Into<Bytes>>(endpoint: A) -> Result<Self> {
        let endpoint = Endpoint::from_shared(endpoint)?;
        let endpoint = endpoint.user_agent(USER_AGENT.as_str())?;
        Ok(Self::new(endpoint))
    }

    /// Overrides the value of [`RuntimeEndpoint`]`::connection_limit`.
    #[inline]
    pub fn connection_limit(mut self, limit: Option<u32>) -> Self {
        self.limit = limit;
        self
    }

    /// Get the endpoint uri.
    #[inline]
    pub fn uri(&self) -> &Uri {
        self.endpoint.uri()
    }
}

impl From<Endpoint> for RuntimeEndpoint {
    #[inline]
    fn from(value: Endpoint) -> Self {
        Self {
            endpoint: value,
            limit: None,
            current: 0,
        }
    }
}

impl TryFrom<Bytes> for RuntimeEndpoint {
    type Error = Error;

    fn try_from(value: Bytes) -> Result<Self, Self::Error> {
        Self::try_new(value)
    }
}

// TODO: Replace with `static USER_AGENT: String` once const `format!` is stable.
static USER_AGENT: LazyLock<String, fn() -> String> = LazyLock::new(format_user_agent);
fn format_user_agent() -> String {
    format!(
        "Axiston/{} (Rust; Ver {})",
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_RUST_VERSION")
    )
}

#[cfg(test)]
mod test {
    use crate::manager::RuntimeEndpoint;
    use crate::Result;

    #[test]
    fn instance() -> Result<()> {
        let addr = "https://example.com/";
        let _endpoint = RuntimeEndpoint::try_new(addr)?;
        Ok(())
    }
}
