use std::ops::{Deref, DerefMut};

use axiston_database::database::Database;
use axiston_database::{Error as SqlError, Result as SqlResult};

use crate::handler::Error;

/// TODO.
#[must_use]
#[derive(Debug, Clone)]
pub struct AppDatabase {
    inner: Database,
}

impl AppDatabase {
    /// Creates a new [`AppDatabase`].
    #[inline]
    pub fn new(inner: Database) -> Self {
        Self { inner }
    }

    /// Returns a new [`AppDatabase`].
    pub async fn connect(addr: impl AsRef<str>) -> SqlResult<Self> {
        Database::connect(addr).await.map(Self::new)
    }
}

impl Deref for AppDatabase {
    type Target = Database;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for AppDatabase {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl From<SqlError> for Error {
    fn from(value: SqlError) -> Self {
        todo!()
    }
}

// impl CheckHealth for AppDatabase {
//     type Error = Error;
// }
