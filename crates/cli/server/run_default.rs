use std::future::Future;
use std::net::{Ipv4Addr, SocketAddr};

use axum::Router;
use tokio::net::TcpListener;

use crate::server::{AppConnectInfo, ServerConfig};

pub async fn run_http_server<F>(
    config: ServerConfig,
    app_router: Router,
    interrupt_fut: F,
) -> anyhow::Result<()>
where
    F: Future<Output = ()> + Send + 'static,
{
    let server_addr = SocketAddr::from((Ipv4Addr::LOCALHOST, config.port));
    let listener = TcpListener::bind(server_addr).await?;
    let app_router = app_router.into_make_service_with_connect_info::<AppConnectInfo>();

    tracing::info!(
        target: "server", port = config.port,
        "server is listening on {}", server_addr,
    );

    axum::serve(listener, app_router)
        .with_graceful_shutdown(interrupt_fut)
        .await?;

    Ok(())
}
