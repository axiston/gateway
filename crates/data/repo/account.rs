use sqlx::PgPool;

/// TODO.
#[derive(Clone)]
pub struct Accounts {
    pool: PgPool,
}

impl Accounts {
    /// Returns a new [`Accounts`].
    #[inline]
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}
