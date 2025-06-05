//!
//! > Nock Nock!!
//!
//! > Whose there?
//!
//! # Openhack Auth
//!
//! Simple auth system which wraps the [argon2] crate.
//!
use argon2::{Argon2, Params};
use std::sync::Arc;

pub use argon2::Error as Argon2Error;
pub use argon2::password_hash::Error as HashError;

#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    /// low level errors from the [argon2] crate
    #[error("{0:?}")]
    Argon2(Argon2Error),

    /// errors around hashing passwords
    #[error("{0:?}")]
    Hasher(HashError),
}

#[derive(Clone)]
pub struct PasswordHasher {
    /// Stores the bytes of the secret which are
    /// referenced by the [Argon2] structure.  The
    /// compiler asserts that this is dead code
    /// because it's never read from for this reason.
    #[allow(dead_code)]
    secret: Arc<[u8]>,

    /// Underlying data structure which implements
    /// the hashing functionality.  Because it refers
    /// to the [PasswordHasher::secret] in the same
    /// struct it can be assigned a lifetime of 'static.
    argon2: Argon2<'static>,
}

impl PasswordHasher {
    /// # Create Password Hasher
    ///
    /// Take care to use the same secret or you
    /// will not be able to verify existing hashes.
    ///
    pub fn new<T>(secret_bytes: T) -> Result<Self, Argon2Error>
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
        );

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
    pub fn create_hash(&self, password: &str) -> Result<String, HashError> {
        use argon2::password_hash::{PasswordHasher as _, SaltString, rand_core};
        let salt = SaltString::generate(&mut rand_core::OsRng);
        Ok(self
            .argon2
            .hash_password(password.as_bytes(), &salt)?
            .to_string())
    }

    /// # Verify Password
    ///
    /// Determine if the password can be verified by a previous
    /// calculated hash.
    ///
    pub fn verify_password(&self, password: &str, hash: &str) -> Result<(), HashError> {
        use argon2::password_hash::{PasswordHash, PasswordVerifier as _};
        let hash = PasswordHash::new(hash)?;
        self.argon2.verify_password(password.as_bytes(), &hash)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn hasher_works() -> Result<(), HashError> {
        let hasher = PasswordHasher::new("roflcopters".as_bytes())?;
        let hash = hasher.create_hash("my-bad-password")?;
        hasher.verify_password("my-bad-password", &hash)?;
        Ok(())
    }

    #[test]
    fn hasher_with_different_secret_does_not_work() -> Result<(), HashError> {
        let hasher_one = PasswordHasher::new("hashone".as_bytes())?;
        let hasher_two = PasswordHasher::new("hashtwo".as_bytes())?;
        let hash = hasher_one.create_hash("my-bad-password")?;
        hasher_one.verify_password("my-bad-password", &hash)?;
        let result = hasher_two.verify_password("my-bad-password", &hash);
        assert!(matches!(
            result.expect_err("hasher error"),
            argon2::password_hash::Error::Password,
        ));
        Ok(())
    }
}
