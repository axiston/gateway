//! TODO.
//!

use std::collections::HashMap;

use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::{get, post, Router};
use serde::{Deserialize, Serialize};

use crate::extract::{Json, RouteVersion};
use crate::handler::{Error, Result};
use crate::service::{AppState, Dataset, Runtime};

/// Verifies the current status of the service.
pub trait CheckHealth {
    /// Associated [`Error`] type.
    type Error: Into<Error>;

    /// Returns `Ok()` if the service is healthy.
    fn check(&self) -> Result<(), Self::Error> {
        Ok(())
    }
}

#[must_use]
#[derive(Debug, Default, Deserialize)]
struct RecentStatusRequest {
    pub verbose: bool,
}

#[must_use]
#[derive(Debug, Serialize)]
struct RecentStatusResponse {
    pub service: bool,
    pub dataset: bool,
    pub runtime: HashMap<String, bool>,
}

/// TODO.
#[tracing::instrument]
async fn recent_status(
    State(dataset): State<Dataset>,
    State(runtime): State<Runtime>,
    request: Option<Json<RecentStatusRequest>>,
) -> Result<(StatusCode, Json<RecentStatusResponse>)> {
    tracing::info!(target: "handler:status", "hello!");
    let Json(request) = request.unwrap_or_default();
    let response = RecentStatusResponse {
        service: dataset.check().is_ok(),
        dataset: dataset.check().is_ok(),
        runtime: HashMap::new(),
    };

    Ok((StatusCode::OK, Json(response)))
}

#[must_use]
#[derive(Debug, Default, Deserialize)]
struct CurrentStatusRequest {
    pub verbose: bool,
}

#[must_use]
#[derive(Debug, Serialize)]
struct CurrentStatusResponse {}

/// TODO.
#[tracing::instrument]
async fn current_status(
    State(dataset): State<Dataset>,
    State(runtime): State<Runtime>,
    RouteVersion(version): RouteVersion,
    Json(request): Json<CurrentStatusRequest>,
) -> Result<(StatusCode, Json<CurrentStatusResponse>)> {
    let response = CurrentStatusResponse {};
    Ok((StatusCode::OK, Json(response)))
}

/// Returns a [`Router`] with all related routes.
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(recent_status))
        .route("/", post(current_status))
}

#[cfg(test)]
mod test {
    use axum_test::TestServer;

    use crate::handler::instance;
    use crate::service::{AppConfig, AppState};

    #[tokio::test]
    async fn routes() -> anyhow::Result<()> {
        let config = AppConfig::default();
        let state = AppState::connect(config).await?;
        let app = instance::routes().with_state(state);
        let server = TestServer::new(app)?;

        Ok(())
    }
}
