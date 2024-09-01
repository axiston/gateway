use sqlx::PgPool;

/// TODO.
#[derive(Clone)]
pub struct Workflows {
    pool: PgPool,
}

impl Workflows {
    /// Returns a new [`Workflows`].
    #[inline]
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}
