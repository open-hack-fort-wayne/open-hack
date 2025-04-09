//!
//! > Nock Nock!!
//!
//! > Whose there?
//!
#![allow(dead_code, unused_variables, unused_imports)]

use argon2::{Argon2, Params};
use std::sync::Arc;

pub use argon2::Error;

#[derive(Debug, thiserror::Error)]
pub enum MyError {
    #[error("{0:?}")]
    Argon2(argon2::Error),

    #[error("{0:?}")]
    Hasher(argon2::password_hash::Error),
}

type Result<T> = std::result::Result<T, MyError>;

#[derive(Clone)]
pub struct PasswordHasher {
    secret: Arc<[u8]>,
    argon2: Argon2<'static>,
}

impl PasswordHasher {
    /// # Create Password Hasher
    ///
    /// Take care to use the same secret or you
    /// will not be able to verify existing hashes.
    ///
    pub fn new<T>(secret_bytes: T) -> Result<Self>
    where
        T: Into<Arc<[u8]>>,
    {
        let secret = secret_bytes.into();
        let raw_ptr = Arc::into_raw(secret);

        let argon2 = Argon2::new_with_secret(
            unsafe { &*raw_ptr },
            argon2::Algorithm::Argon2id,
            argon2::Version::V0x13,
            Params::default(),
        )
        .map_err(MyError::Argon2);

        let secret = unsafe { Arc::from_raw(raw_ptr) };

        Ok(Self {
            secret,
            argon2: argon2?,
        })
    }

    /// # Create Hash
    ///
    /// Returns a string in the "PHC" format.
    ///
    /// ```text
    /// $<id>[$v=<version>][$<param>=<value>(,<param>=<value>)*][$<salt>[$<hash>]
    /// ```
    ///
    pub fn create_hash(&self, password: &str) -> Result<String> {
        use argon2::password_hash::{PasswordHasher as _, SaltString, rand_core};
        let salt = SaltString::generate(&mut rand_core::OsRng);
        Ok(self
            .argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(MyError::Hasher)?
            .to_string())
    }

    /// # Verify Password
    ///
    /// Determine if the password can be verified by a previous
    /// calculated hash.
    ///
    pub fn verify_password(&self, password: &str, hash: &str) -> Result<()> {
        use argon2::password_hash::{PasswordHash, PasswordVerifier as _};
        let hash = PasswordHash::new(hash).map_err(MyError::Hasher)?;
        self.argon2
            .verify_password(password.as_bytes(), &hash)
            .map_err(MyError::Hasher)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn hasher_works() -> Result<()> {
        let hasher = PasswordHasher::new("roflcopters".as_bytes())?;
        let hash = hasher.create_hash("my-bad-password")?;
        hasher.verify_password("my-bad-password", &hash)?;
        Ok(())
    }

    #[test]
    fn hasher_with_different_secret_does_not_work() -> Result<()> {
        let hasher_one = PasswordHasher::new("hashone".as_bytes())?;
        let hasher_two = PasswordHasher::new("hashtwo".as_bytes())?;
        let hash = hasher_one.create_hash("my-bad-password")?;
        hasher_one.verify_password("my-bad-password", &hash)?;
        let result = hasher_two.verify_password("my-bad-password", &hash);
        assert!(matches!(
            MyError::Hasher(argon2::password_hash::Error::Password),
            result
        ));
        Ok(())
    }
}
