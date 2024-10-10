use axiston_database::Error as DbError;

use crate::handler::Error;

// TODO: Split into handlers.
impl From<DbError> for Error {
    fn from(value: DbError) -> Self {
        // let constraint = value.constraint();
        todo!()
    }
}
