//! TODO.
//!

use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::{get, Router};
use serde::{Deserialize, Serialize};

use crate::extract::Json;
#[cfg(feature = "support-invite")]
use crate::handler::account_invite;
#[cfg(feature = "support-oauth2")]
use crate::handler::account_oauth2;
use crate::handler::Result;
use crate::service::{AppState, Dataset};

#[must_use]
#[derive(Debug, Deserialize)]
struct AuthMethodsRequest {}

#[must_use]
#[derive(Debug, Serialize)]
struct AuthMethodsResponse {
    pub oauth2: bool,
    pub invite: bool,
}

/// TODO.
#[tracing::instrument]
async fn retrieve_auth_methods(
    State(dataset): State<Dataset>,
    Json(request): Json<AuthMethodsRequest>,
) -> Result<(StatusCode, Json<AuthMethodsResponse>)> {
    let response = AuthMethodsResponse {
        oauth2: cfg!(feature = "support-oauth2"),
        invite: cfg!(feature = "support-invite"),
    };

    Ok((StatusCode::OK, Json(response)))
}

/// Returns a [`Router`] with all related routes.
pub fn routes() -> Router<AppState> {
    let router = Router::new().route("/auth", get(retrieve_auth_methods));

    #[cfg(feature = "support-oauth2")]
    let router = router.nest("/oauth2", account_oauth2::routes());
    #[cfg(feature = "support-invite")]
    let router = router.nest("/invite", account_invite::routes());
    router
}

#[cfg(test)]
mod test {
    use axum_test::TestServer;

    use crate::handler::account;
    use crate::service::{AppConfig, AppState};

    #[tokio::test]
    async fn routes() -> anyhow::Result<()> {
        let config = AppConfig::default();
        let state = AppState::connect(config).await?;
        let app = account::routes().with_state(state);
        let server = TestServer::new(app)?;

        Ok(())
    }
}
