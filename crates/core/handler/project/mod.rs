//! TODO.
//!

use axum::Router;
use serde::Deserialize;
use ts_rs::TS;

use crate::service::AppState;

mod analytics;
mod integrations;
mod members;
mod projects;
mod settings;
mod webhooks;

/// `Path` param for a `:project` router.
///
/// `./:account/:project/`
#[must_use]
#[derive(Debug, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "project.ts")]
pub struct ProjectPathParams {
    pub account: String,
    pub project: String,
}

/// Returns a [`Router`] with all related routes.
pub fn routes() -> Router<AppState> {
    Router::new()
        .merge(integrations::routes())
        .merge(members::routes())
        .merge(projects::routes())
        .merge(settings::routes())
        .merge(webhooks::routes())
}

#[cfg(test)]
mod test {
    use axum_test::TestServer;

    use crate::handler::project;
    use crate::service::{AppConfig, AppState};

    #[tokio::test]
    async fn routes() -> anyhow::Result<()> {
        let config = AppConfig::default();
        let state = AppState::connect(config).await?;
        let app = project::routes().with_state(state);
        let server = TestServer::new(app)?;

        Ok(())
    }
}
