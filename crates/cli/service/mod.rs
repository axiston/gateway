//! TODO.
//!

use axiston_data::database::Database;

pub use crate::service::app_config::{AppBuilder, AppConfig};
pub use crate::service::app_database::AppDatabase;
pub use crate::service::state_dataset::Dataset;
pub use crate::service::state_runtime::Runtime;

mod app_config;
mod app_database;
mod state_dataset;
mod state_runtime;

/// Application state.
///
/// Used for the [`State`] extraction (dependency injection).
///
/// [`State`]: axum::extract::State
#[derive(Debug, Clone)]
#[must_use = "state does nothing unless you use it"]
pub struct AppState {
    dataset: Dataset,
    database: Database,
    runtime: Runtime,
}

impl AppState {
    /// Returns a new [`AppState`].
    #[inline]
    pub async fn connect(config: AppConfig) -> anyhow::Result<Self> {
        Ok(Self {
            dataset: Dataset::new(),
            database: Database::connect(config.database).await?,
            runtime: Runtime::new(),
        })
    }
}

macro_rules! impl_di {
    ($($f:ident: $t:ty),+) => {$(
        impl axum::extract::FromRef<AppState> for $t {
            fn from_ref(state: &AppState) -> Self {
                state.$f.clone()
            }
        }
    )+};
}

impl_di!(dataset: Dataset);
impl_di!(runtime: Runtime);

#[cfg(test)]
mod test {
    use crate::service::{AppConfig, AppState};

    #[test]
    fn instance_app_state() {
        let config = AppConfig::builder();
        let _ = AppState::connect(config.build());
    }

    #[test]
    fn configure_app_state() {
        let config = AppConfig::builder();
        let _ = AppState::connect(config.build());
    }
}
