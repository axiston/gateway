use std::time::Duration;

use axiston_database::AppDatabase;
use tokio::task::JoinHandle;

use crate::service::AppState;

/// TODO.
#[derive(Debug)]
pub struct SchedulerError {}

/// Runs all active triggers sequentially in the priority order.
#[derive(Debug)]
pub struct SchedulerRuntime {
    query_every: Option<Duration>,
    app_database: AppDatabase,
}

impl SchedulerRuntime {
    /// Returns a new [`SchedulerRuntime`].
    #[inline]
    pub fn new(app_state: AppState) -> Self {
        Self {
            query_every: None,
            app_database: app_state.app_database,
        }
    }

    pub fn run_trigger_loop(self) -> JoinHandle<Result<(), SchedulerError>> {
        tracing::info!(target: "scheduler", "waiting");
        let handle = tokio::spawn(async move {
            tracing::info!(target: "scheduler", "running");
            self.execute_loop().await
        });

        handle
    }

    async fn execute_loop(&self) -> Result<(), SchedulerError> {
        loop {
            self.execute_once().await?;
            if let Some(query_every) = self.query_every {
                tokio::time::sleep(query_every).await;
            }
        }
    }

    async fn execute_once(&self) -> Result<(), SchedulerError> {
        todo!()
    }
}

#[cfg(test)]
mod test {}

// TODO GraphWorker
