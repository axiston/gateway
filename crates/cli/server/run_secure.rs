use std::future::Future;
use std::net::{Ipv4Addr, SocketAddr};

use axiston_server::service::AppConnectInfo;
use axum::Router;
use axum_server::tls_rustls::RustlsConfig;
use axum_server::Handle;
use tokio::task::JoinHandle;

use crate::server::{run_redirect_server, ServerConfig};

pub fn run_redirect_daemon<F>(
    http_port: u16,
    https_port: u16,
    interrupt_fut: F,
) -> JoinHandle<anyhow::Result<()>>
where
    F: Future<Output = ()> + Send + 'static,
{
    // TODO(bug): Fix error return (e.g. port was already taken).
    tokio::spawn(run_redirect_server(http_port, https_port, interrupt_fut))
}

async fn interrupt_handle<F>(shutdown_handle: Handle, interrupt_fut: F)
where
    F: Future<Output = ()> + Send + 'static,
{
    interrupt_fut.await;
    shutdown_handle.shutdown();
}

pub async fn run_https_server<F>(
    config: ServerConfig,
    app_router: Router,
    interrupt_fut: F,
) -> anyhow::Result<()>
where
    F: Future<Output = ()> + Send + 'static,
{
    let shutdown_handle = Handle::new();
    let interrupt_fut = interrupt_handle(shutdown_handle.clone(), interrupt_fut);
    let tls_config = RustlsConfig::from_pem_file(config.cert, config.key).await?;
    let app_router = app_router.into_make_service_with_connect_info::<AppConnectInfo>();
    let daemon_handle = run_redirect_daemon(config.redirect, config.port, interrupt_fut);
    let server_addr = SocketAddr::from((Ipv4Addr::LOCALHOST, config.port));

    tracing::info!(
        target: "server", port = config.port,
        "server is listening on {}", server_addr,
    );

    axum_server::bind_rustls(server_addr, tls_config)
        .handle(shutdown_handle)
        .serve(app_router)
        .await?;

    let handle_result = daemon_handle.await?;
    let _ = handle_result?;
    Ok(())
}
