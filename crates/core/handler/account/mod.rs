//! TODO.
//!

use axum::Router;
use serde::Deserialize;
use ts_rs::TS;

use crate::service::AppState;

mod accounts;
mod emails;
mod invitation;
mod password;

/// `Path` param for a `:account` router.
///
/// `./:account/`
#[must_use]
#[derive(Debug, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "account.ts")]
pub struct AccountPathParams {
    pub account: String,
}

/// Returns a [`Router`] with all related routes.
pub fn routes() -> Router<AppState> {
    Router::new()
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
