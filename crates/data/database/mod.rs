//! TODO.
//!

use std::fmt;

use sqlx::PgPool;
use crate::repo::{Accounts, Projects, Workflows};
use crate::Result;

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
    pub async fn connect(url: impl AsRef<str>) -> Result<Self> {
        let pool = PgPool::connect(url.as_ref()).await;
        pool.map(Self::new).map_err(Into::into)
    }

    /// Returns the underlying database pool.
    #[inline]
    pub fn pool(&self) -> PgPool {
        self.pool.clone()
    }

    /// Returns a new [`Accounts`].
    #[inline]
    pub fn accounts(&self) -> Accounts {
        Accounts::new(self.pool())
    }

    /// Returns a new [`Projects`].
    #[inline]
    pub fn projects(&self) -> Projects {
        Projects::new(self.pool())
    }

    /// Returns a new [`Workflows`].
    #[inline]
    pub fn workflows(&self) -> Workflows {
        Workflows::new(self.pool())
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
