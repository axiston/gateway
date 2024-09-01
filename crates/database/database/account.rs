use sqlx::PgPool;

use crate::Result;

#[must_use]
#[derive(Debug, thiserror::Error)]
pub enum AccountError {}

/// TODO.
#[derive(Clone)]
pub struct AccountsDatabase {
    pool: PgPool,
}

impl AccountsDatabase {
    /// Returns a new [`AccountsDatabase`].
    #[inline]
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// TODO.
    pub async fn create_account(&self, account_name: &str, password: &str) -> Result<()> {
        // sqlx::Error::Database()
        Ok(())
    }

    /// TODO.
    pub async fn create_session(&self) -> Result<()> {
        Ok(())
    }

    /// TODO.
    pub async fn verify_account(&self, session_token: &str) -> Result<()> {
        Ok(())
    }

    /// TODO.
    pub async fn update_account_username(&self) -> Result<()> {
        Ok(())
    }

    /// TODO.
    pub async fn update_password_account(&self) -> Result<()> {
        Ok(())
    }

    /// TODO.
    pub async fn delete_account(&self, account_name: &str) -> Result<()> {
        Ok(())
    }
}

pub trait AccountRepo {}
