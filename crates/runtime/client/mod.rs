use std::sync::LazyLock;

use derive_more::From;
use tonic::transport::{Channel, Endpoint};

pub use crate::client::client_builder::RuntimeConnBuilder;
use crate::client::runtime_proto::runtime_client::RuntimeClient;
use crate::RuntimeResult;

mod client_builder;
mod recycle_method;

pub mod runtime_proto {
    tonic::include_proto!("runtime");
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

/// TODO.
#[derive(Debug, From, thiserror::Error)]
#[must_use = "errors do nothing unless you use them"]
pub enum RuntimeConnError {
    /// Transport failure (from the client or server).
    #[error("transport failure: {0}")]
    Transport(tonic::transport::Error),
}

#[derive(Debug, Clone)]
#[must_use = "clients do nothing unless you use them"]
pub struct RuntimeConn {
    client: RuntimeClient<Channel>,
}

impl RuntimeConn {
    /// Returns a new [`RuntimeConn`].
    #[inline]
    pub fn new(client: RuntimeClient<Channel>) -> Self {
        Self { client }
    }

    /// Returns a new [`RuntimeConn`].
    pub async fn connect(conn: &str) -> RuntimeResult<Self> {
        let endpoint = Endpoint::from_shared(conn.to_owned())?;
        let endpoint = endpoint.user_agent(USER_AGENT.as_str())?;
        let client = RuntimeClient::connect(endpoint).await?;
        Ok(Self { client })
    }

    /// Returns a new [`RuntimeConnBuilder`].
    #[inline]
    pub fn builder() -> RuntimeConnBuilder {
        RuntimeConnBuilder::new()
    }
}
