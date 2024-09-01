#![forbid(unsafe_code)]

use std::time::Duration;

use axum::Router;
use clap::Parser;

use crate::middleware::{initialize_tracing, RouterExt};
use crate::server::ServerConfig;
use crate::service::{AppConfig, AppState};

mod extract;
mod handler;
mod middleware;
mod server;
mod service;

/// Command-line arguments.
#[derive(Debug, Clone, Parser)]
pub struct Args {
    /// Bound server port.
    #[arg(short, long, default_value_t = 3000)]
    pub port: u16,

    /// Database connection string.
    #[arg(short, long, default_value = "postgresql://usr:pwd@localhost:5432/db")]
    pub database: String,

    /// Bound (redirect) server port.
    #[cfg(feature = "support-https")]
    #[arg(short, long, default_value_t = 3001)]
    pub redirect: u16,

    /// Directory containing `cert.pem` and `key.pem` files.
    #[cfg(feature = "support-https")]
    #[arg(short, long)]
    pub keys: std::path::PathBuf,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    initialize_tracing().await?;

    // Service.
    let config = AppConfig::from(args.clone());
    let state = AppState::connect(config).await?;

    let app = Router::new()
        .nest("/account", handler::project::routes())
        .nest("/instance", handler::instance::routes())
        .nest("/project", handler::project::routes())
        .nest("/workflow", handler::workflow::routes())
        .fallback(handler::fallback)
        .with_error_handling_layer(Duration::from_secs(60))
        .with_observability_layer()
        .with_state(state);

    // Listen.
    let config = ServerConfig::from(args);
    let timeout = Duration::from_secs(2);
    let fut = server::shutdown_signal(timeout);

    #[cfg(not(feature = "support-https"))]
    server::run_http_server(config, app, fut).await?;
    #[cfg(feature = "support-https")]
    server::run_https_server(config, app, fut).await?;

    Ok(())
}
