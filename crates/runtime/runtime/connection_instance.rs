use std::fmt;

use tonic::transport::{Channel, Endpoint};

use crate::runtime::connection_instance::runtime_proto::runtime_client::RuntimeClient;
use crate::runtime::connection_instance::runtime_proto::HelloRequest;
use crate::Result;

pub mod runtime_proto {
    tonic::include_proto!("runtime");
}

/// TODO.
#[must_use]
pub struct ConnectionInstance {
    client: RuntimeClient<Channel>,
}

impl ConnectionInstance {
    /// Returns a new [`ConnectionInstance`].
    pub async fn connect(conn: &str) -> Result<Self> {
        let endpoint = Endpoint::from_shared(conn.to_owned()).unwrap();
        let endpoint = endpoint.user_agent("runtime").unwrap();

        let client = RuntimeClient::connect(endpoint).await.unwrap();

        Ok(Self { client })
    }

    /// TODO.
    pub async fn check(&mut self) -> Result<()> {
        let x = self.client.hello(HelloRequest::default()).await;
        Ok(())
    }
}

impl fmt::Debug for ConnectionInstance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use crate::Result;

    #[test]
    fn build_from_address() -> Result<()> {
        Ok(())
    }
}
