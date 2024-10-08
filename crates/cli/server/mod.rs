//! TODO.
//!

use std::time::Duration;

use axum::Router;

use crate::server::inter_signal::shutdown_signal;
#[cfg(not(feature = "support-https"))]
use crate::server::run_default::run_http_server;
#[cfg(feature = "support-https")]
use crate::server::run_redirect::run_redirect_server;
#[cfg(feature = "support-https")]
use crate::server::run_secure::run_https_server;
pub use crate::server::serv_config::{ServerBuilder, ServerConfig};

mod inter_signal;
mod run_default;
#[cfg(feature = "support-https")]
mod run_redirect;
#[cfg(feature = "support-https")]
mod run_secure;
mod serv_config;

/// Runs the supported (`http` or `https`) server.
///
/// - Runs a simple `http` server by default.
/// - Runs `https` server if the `support-http` feature is enabled.
/// - Runs `redirect` server if the `support-http` feature is enabled.
pub async fn run_supported_server(
    server_config: ServerConfig,
    app_router: Router,
) -> anyhow::Result<()> {
    let timeout = Duration::from_secs(60);
    let fut = shutdown_signal(timeout);

    #[cfg(not(feature = "support-https"))]
    run_http_server(server_config, app_router, fut).await?;
    #[cfg(feature = "support-https")]
    run_https_server(server_config, app_router, fut).await?;
    Ok(())
}
