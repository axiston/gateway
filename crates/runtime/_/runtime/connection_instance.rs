use std::fmt;
use std::sync::LazyLock;
use tonic::transport::{Channel, Endpoint};

use crate::runtime::connection_instance::runtime_proto::runtime_client::RuntimeClient;
use crate::runtime::connection_instance::runtime_proto::HelloRequest;
use crate::RuntimeResult;

pub mod runtime_proto {
    tonic::include_proto!("runtime");
}

/// TODO.
#[must_use]
pub struct ConnectionInstance {
    client: RuntimeClient<Channel>,
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

impl ConnectionInstance {
    /// Returns a new [`ConnectionInstance`].
    pub async fn connect(conn: &str) -> RuntimeResult<Self> {
        let endpoint = Endpoint::from_shared(conn.to_owned())?;
        let endpoint = endpoint.user_agent(USER_AGENT.as_str())?;
        let client = RuntimeClient::connect(endpoint).await?;
        Ok(Self { client })
    }

    /// Returns the underlying (generated) runtime client.
    #[inline]
    pub fn as_inner(&self) -> RuntimeClient<Channel> {
        self.client.clone()
    }

    /// TODO.
    pub async fn check(&mut self) -> RuntimeResult<()> {
        let x = self.client.hello(HelloRequest::default()).await;
        Ok(())
    }
}

impl fmt::Debug for ConnectionInstance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Connection").finish_non_exhaustive()
    }
}

#[cfg(test)]
mod test {
    use crate::RuntimeResult;

    #[test]
    fn build_from_address() -> RuntimeResult<()> {
        Ok(())
    }
}
