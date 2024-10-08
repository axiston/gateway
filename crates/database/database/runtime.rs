use sqlx::PgPool;

use crate::Result;

/// TODO.
#[derive(Clone)]
pub struct RuntimesDatabase {
    pool: PgPool,
}

impl RuntimesDatabase {
    /// Returns a new [`RuntimesDatabase`].
    #[inline]
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// TODO.
    pub async fn try_register_runtime(&self) -> Result<String> {
        Ok("".to_owned())
    }
}
