//! All handlers for a `:workflows` router.
//!

mod analytics;
mod editor;

use axum::extract::Request;
use axum::middleware::Next;
use axum::response::Response;
use axum::routing::Router;
use serde::Deserialize;
use ts_rs::TS;
use uuid::Uuid;

use crate::extract::{AuthState, Path};
use crate::handler::projects::ProjectPathParams;
use crate::handler::Result;
use crate::service::AppState;

/// `Path` param for a `:project` router.
///
/// #### Endpoints
///
/// - `./projects/:project/`
#[must_use]
#[derive(Debug, Deserialize, TS)]
#[ts(export, export_to = "workflows.ts")]
pub struct WorkflowPathParams {
    /// Unique identifier of the project.
    pub workflow: Uuid,
}

/// Interrupts the request if the account is not allowed to perform an action on
/// the requested project.
async fn verify_workflow_access(
    authentication: AuthState,
    Path(params): Path<ProjectPathParams>,
    request: Request,
    next: Next,
) -> Result<Response> {
    Ok(next.run(request).await)
}

/// Returns a [`Router`] with all related routes.
pub fn routes(state: AppState) -> Router<AppState> {
    Router::new()
}

#[cfg(test)]
mod test {
    use axum_test::TestServer;

    use crate::handler::workflows::routes;
    use crate::service::{AppConfig, AppState};

    #[tokio::test]
    async fn routes() -> anyhow::Result<()> {
        let config = AppConfig::default();
        let state = AppState::connect(config).await?;
        let app = routes().with_state(state);
        let server = TestServer::new(app)?;

        Ok(())
    }
}
