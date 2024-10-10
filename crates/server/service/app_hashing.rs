use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use rand::rngs::OsRng;

use crate::handler::{ErrorKind, Result};

/// TODO.
#[derive(Debug, Clone)]
pub struct AppHashing {
    argon2_instance: Argon2<'static>,
}

impl AppHashing {
    /// Returns a new [`AppHashing`].
    #[inline]
    pub fn new() -> Self {
        Self {
            argon2_instance: Argon2::default(),
        }
    }

    /// TODO.
    pub fn hash_password(&self, password: &str) -> Result<String> {
        let salt = SaltString::generate(&mut OsRng);
        match self
            .argon2_instance
            .hash_password(password.as_bytes(), &salt)
        {
            Ok(password_hash) => Ok(password_hash.to_string()),
            Err(_) => Err(ErrorKind::InternalServerError.into()),
        }
    }

    /// TODO.
    pub fn verify_password(&self, password: &str, password_hash: &str) -> Result<()> {
        let password_buf = password.as_bytes();
        let Ok(parsed_hash) = PasswordHash::new(&password_hash) else {
            return Err(ErrorKind::InternalServerError.into());
        };

        if self
            .argon2_instance
            .verify_password(password_buf, &parsed_hash)
            .is_err()
        {
            // TODO: BadPassword error kind.
            return Err(ErrorKind::Unauthorized.into());
        };

        Ok(())
    }
}
