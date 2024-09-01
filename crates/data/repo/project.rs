use sqlx::PgPool;
use crate::Result;

/// TODO.
#[derive(Clone)]
pub struct Projects {
    pool: PgPool,
}

impl Projects {
    /// Returns a new [`Projects`].
    #[inline]
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// TODO.
    pub fn create(&self) -> Result<()> {
        Ok(())
    }

    /// TODO.
    pub fn list(&self, account: &str) -> Result<()> {
        Ok(())
    }

    /// TODO.
    pub fn retrieve(&self, account: &str, project: &str) -> Result<()> {
        Ok(())
    }

    /// TODO.
    pub fn delete(&self) -> Result<()> {
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Project {
    pub account: String,
    pub name: String,
    pub tags: Vec<String>,
    pub archive: bool,
}

impl Project {}
