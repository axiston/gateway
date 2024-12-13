#![forbid(unsafe_code)]

mod config;
mod middleware;
mod server;

use std::time::Duration;

use axiston_server::handler::routes;
use axiston_server::middleware::RouterTracingExt;
use axiston_server::service::{AppState, SchedulerRuntime};
use axum::Router;
use clap::Parser;

use crate::config::Args;
use crate::middleware::initialize_tracing;
use crate::server::run_supported_server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Config.
    let args = Args::parse();
    initialize_tracing().await?;

    // Service.
    let app_config = args.build_app_config();
    let state = AppState::connect(app_config).await?;

    let scheduler = SchedulerRuntime::new(state.clone());
    let scheduler_handler = scheduler.run_trigger_loop();

    let app = Router::new()
        .merge(routes())
        .with_inner_error_handling_layer(Duration::from_secs(60))
        .with_inner_observability_layer()
        .with_state(state);

    // Listen.
    let server_config = args.build_server_config();
    run_supported_server(server_config, app).await?;
    let _ = scheduler_handler.await?;

    Ok(())
}
