use sqlx::PgPool;

/// TODO.
#[derive(Clone)]
pub struct WebhookDatabase {
    pool: PgPool,
}

impl WebhookDatabase {
    /// Returns a new [`WebhookDatabase`].
    #[inline]
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}
