//! TODO.
//!

use axum::routing::Router;

use crate::service::AppState;

/// Returns a [`Router`] with all related routes.
pub fn routes() -> Router<AppState> {
    Router::new()
}

#[cfg(test)]
mod test {
    use axum_test::TestServer;

    use crate::handler::account_oauth2;
    use crate::service::{AppConfig, AppState};

    #[tokio::test]
    async fn routes() -> anyhow::Result<()> {
        let config = AppConfig::default();
        let state = AppState::connect(config).await?;
        let app = account_oauth2::routes().with_state(state);
        let server = TestServer::new(app)?;

        Ok(())
    }
}
