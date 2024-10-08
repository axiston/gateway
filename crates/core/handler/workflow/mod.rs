//! TODO.
//!

use axum::Router;
use serde::Deserialize;
use ts_rs::TS;

use crate::service::AppState;

mod editor;
mod executions;
mod revisions;
mod workflows;

/// [`Path`] param for a `:workflow` router.
///
/// `./:account/:project/:workflow/`
///
/// [`Path`]: crate::extract::Path
#[must_use]
#[derive(Debug, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "account.ts")]
pub struct WorkflowPathParams {
    pub account: String,
    pub project: String,
    pub workflow: String,
}

/// Returns a [`Router`] with all related routes.
pub fn routes() -> Router<AppState> {
    Router::new()
        .merge(editor::routes())
        .merge(executions::routes())
        .merge(revisions::routes())
        .merge(workflows::routes())
}

#[cfg(test)]
mod test {
    use axum_test::TestServer;

    use crate::handler::workflow;
    use crate::service::{AppConfig, AppState};

    #[tokio::test]
    async fn routes() -> anyhow::Result<()> {
        let config = AppConfig::default();
        let state = AppState::connect(config).await?;
        let app = workflow::routes().with_state(state);
        let server = TestServer::new(app)?;

        Ok(())
    }
}
