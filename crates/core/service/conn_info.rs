use std::net::SocketAddr;

use axum::extract::connect_info::Connected;
use axum::serve::IncomingStream;

// TODO: Move into ./server

/// Produces information about the connection.
#[derive(Debug, Clone)]
#[must_use]
pub struct AppConnectInfo {
    pub addr: SocketAddr,
}

impl Connected<IncomingStream<'_>> for AppConnectInfo {
    fn connect_info(target: IncomingStream<'_>) -> Self {
        let addr = SocketAddr::connect_info(target);
        Self { addr }
    }
}

#[cfg(feature = "support-https")]
// https://github.com/programatik29/axum-server/issues/12
impl Connected<SocketAddr> for AppConnectInfo {
    fn connect_info(addr: SocketAddr) -> Self {
        Self { addr }
    }
}

#[cfg(test)]
mod test {
    use axum::extract::ConnectInfo;
    use axum::routing::{any, Router};
    use axum_test::TestServer;

    use crate::handler::Result;
    use crate::service::AppConnectInfo;

    async fn handler(ConnectInfo(_): ConnectInfo<AppConnectInfo>) -> Result<()> {
        Ok(())
    }

    #[tokio::test]
    async fn extract() -> anyhow::Result<()> {
        let app = Router::new().route("/", any(handler));
        let server = TestServer::new(app)?;
        let _ = server.get("/").await;

        Ok(())
    }
}
