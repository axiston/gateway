//! TODO.
//!

use std::fmt;

use sqlx::PgPool;

pub use crate::database::account::AccountsDatabase;
pub use crate::database::project::ProjectsDatabase;
pub use crate::database::runtime::RuntimesDatabase;
pub use crate::database::webhook::WebhookDatabase;
pub use crate::database::workflow::WorkflowsDatabase;
use crate::Result;

mod account;
mod config;
mod project;
mod runtime;
mod webhook;
mod workflow;

/// TODO.
#[derive(Clone)]
pub struct Database {
    pool: PgPool,
}

impl Database {
    /// Returns a new [`Database`].
    #[inline]
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Connects to the database and returns a new [`Database`].
    pub async fn connect(addr: impl AsRef<str>) -> Result<Self> {
        let pool = PgPool::connect(addr.as_ref()).await;
        pool.map(Self::new).map_err(Into::into)
    }

    /// Returns the underlying database pool.
    #[inline]
    pub fn pool(&self) -> PgPool {
        self.pool.clone()
    }

    /// Returns a new [`AccountsDatabase`].
    #[inline]
    pub fn accounts(&self) -> AccountsDatabase {
        AccountsDatabase::new(self.pool())
    }

    /// Returns a new [`ProjectsDatabase`].
    #[inline]
    pub fn projects(&self) -> ProjectsDatabase {
        ProjectsDatabase::new(self.pool())
    }

    /// Returns a new [`RuntimesDatabase`].
    #[inline]
    pub fn runtimes_database(&self) -> RuntimesDatabase {
        RuntimesDatabase::new(self.pool())
    }

    /// Returns a new [`WebhookDatabase`].
    #[inline]
    pub fn webhook(&self) -> WebhookDatabase {
        WebhookDatabase::new(self.pool())
    }

    /// Returns a new [`WorkflowsDatabase`].
    #[inline]
    pub fn workflows(&self) -> WorkflowsDatabase {
        WorkflowsDatabase::new(self.pool())
    }
}

impl fmt::Debug for Database {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Database").finish_non_exhaustive()
    }
}

#[cfg(test)]
mod test {
    use crate::Result;

    #[test]
    fn build() -> Result<()> {
        Ok(())
    }
}
