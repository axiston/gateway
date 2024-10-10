use sea_orm::SqlErr;

/// TODO.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub enum ConstraintViolation {
    #[default]
    Unknown,
}

impl ConstraintViolation {
    /// Parses an error from unsuccessful SQL query.
    pub fn new(error: SqlErr) -> Self {
        match error {
            SqlErr::UniqueConstraintViolation(constraint) => {
                Self::from_unique_constraint_violation(constraint.as_str())
            }
            SqlErr::ForeignKeyConstraintViolation(constraint) => {
                Self::from_foreign_constraint_violation(constraint.as_str())
            }
            _ => Self::Unknown,
        }
    }

    /// Parses a [`ConstraintViolation`] from a [`SqlErr::UniqueConstraintViolation`].
    fn from_unique_constraint_violation(constraint: &str) -> Self {
        match constraint {
            "" => Self::Unknown,
            _ => Self::Unknown,
        }
    }

    /// Parses a [`ConstraintViolation`] from a [`SqlErr::ForeignKeyConstraintViolation`].
    fn from_foreign_constraint_violation(constraint: &str) -> Self {
        match constraint {
            "" => Self::Unknown,
            _ => Self::Unknown,
        }
    }
}

impl From<SqlErr> for ConstraintViolation {
    #[inline]
    fn from(value: SqlErr) -> Self {
        Self::new(value)
    }
}
