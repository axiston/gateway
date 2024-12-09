//! All handlers for a `:project` router.
//!

mod members;
mod projects;
mod webhooks;

use axiston_database::AppDatabase;
use axum::extract::{Request, State};
use axum::middleware::{from_fn_with_state, Next};
use axum::response::Response;
use axum::routing::Router;
use serde::Deserialize;
use ts_rs::TS;
use uuid::Uuid;

use crate::extract::{AuthState, Path};
use crate::handler::Result;
use crate::service::AppState;

/// `Path` param for a `:project` router.
///
/// #### Endpoints
///
/// - `./projects/:project/`
#[must_use]
#[derive(Debug, Deserialize, TS)]
#[ts(export, export_to = "projects.ts")]
pub struct ProjectPathParams {
    /// Unique identifier of the project.
    pub project: Uuid,
}

/// Interrupts the request if the account is not allowed to perform an action on
/// the requested project.
async fn verify_project_access(
    authentication: AuthState,
    State(database): State<AppDatabase>,
    Path(params): Path<ProjectPathParams>,
    request: Request,
    next: Next,
) -> Result<Response> {
    // TODO.
    // let _ = database.find_member(params.project).await?;
    Ok(next.run(request).await)
}

/// Returns a [`Router`] with all related routes.
pub fn routes(state: AppState) -> Router<AppState> {
    let verify_project_access = from_fn_with_state(state, verify_project_access);
    Router::new()
        .merge(projects::routes())
        .merge(members::routes())
        .merge(webhooks::routes())
        // Authorization:
        .route_layer(verify_project_access)
}

#[cfg(test)]
mod test {
    use axum_test::TestServer;

    use crate::handler::projects::routes;
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
