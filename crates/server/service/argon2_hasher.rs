use argon2::password_hash::{Error, SaltString};
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use derive_more::From;
use rand::rngs::OsRng;

use crate::handler::{ErrorKind, Result};

/// Implements an `argon2` password hashing function.
#[derive(Debug, Default, Clone, From)]
pub struct Argon2Hasher {
    inner: Argon2<'static>,
}

impl Argon2Hasher {
    /// Returns a new [`Argon2Hasher`].
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the password hash of the provided password.
    pub fn hash_password(&self, password: &str) -> Result<String> {
        let salt = SaltString::generate(&mut OsRng);
        match self.inner.hash_password(password.as_bytes(), &salt) {
            Ok(password_hash) => Ok(password_hash.to_string()),
            Err(_) => Err(ErrorKind::InternalServerError.into()),
        }
    }

    /// Returns `Ok()` if the password was successfully verified.
    pub fn verify_password(&self, password: &str, password_hash: &str) -> Result<()> {
        let Ok(parsed_hash) = PasswordHash::new(&password_hash) else {
            return Err(ErrorKind::InternalServerError.into());
        };

        let password_buf = password.as_bytes();
        match self.inner.verify_password(password_buf, &parsed_hash) {
            Ok(_) => Ok(()),
            Err(Error::Password) => Err(ErrorKind::Unauthorized.into()),
            Err(_) => Err(ErrorKind::InternalServerError.into()),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::handler::Result;
    use crate::service::Argon2Hasher;

    const PASSWORD: &str = "qwerty12345";

    #[test]
    fn hash_password() -> Result<()> {
        let hasher = Argon2Hasher::new();
        let _ = hasher.hash_password(PASSWORD)?;
        Ok(())
    }

    #[test]
    fn verify_password() -> Result<()> {
        let hasher = Argon2Hasher::new();
        let hash = hasher.hash_password(PASSWORD)?;
        hasher.verify_password(PASSWORD, &hash)?;
        Ok(())
    }
}
