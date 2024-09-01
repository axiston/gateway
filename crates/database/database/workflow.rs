use sqlx::PgPool;

/// TODO.
#[derive(Clone)]
pub struct WorkflowsDatabase {
    pool: PgPool,
}

impl WorkflowsDatabase {
    /// Returns a new [`WorkflowsDatabase`].
    #[inline]
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}
