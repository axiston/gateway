#![forbid(unsafe_code)]

mod config;
mod server;

use axiston_core::handler::routes;
use axiston_core::middleware::{initialize_tracing, RouterExt};
use axiston_core::service::{AppConfig, AppState};
use axum::Router;
use clap::Parser;

use crate::config::Args;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    initialize_tracing().await?;

    // Service.
    let config = AppConfig::default();
    // TODO: Construct AppConfig from Args.
    let state = AppState::connect(config).await?;

    let app = Router::new()
        .merge(routes())
        // .with_error_handling_layer(Duration::from_secs(60))
        // .with_observability_layer()
        .with_state(state);

    // Listen.
    let config = server::ServerConfig::default();
    // TODO: Construct ServerConfig from Args.
    server::run_supported_server(config, app).await?;

    Ok(())
}
