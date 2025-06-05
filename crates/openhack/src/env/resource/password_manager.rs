use super::*;
use openhack_auth::PasswordHasher;

pub use openhack_auth::HashError;

pub trait PasswordManager {
    fn get_password_hasher(&self) -> &PasswordHasher;

    fn verify_password(&self, password: &str, hash: &str) -> Result<(), HashError> {
        self.get_password_hasher().verify_password(password, hash)
    }

    fn hash_password(&self, password: &str) -> Result<String, HashError> {
        self.get_password_hasher().create_hash(password)
    }
}

impl<T> PasswordManager for T
where
    T: Resource<PasswordHasher>,
{
    fn get_password_hasher(&self) -> &PasswordHasher {
        self.as_res()
    }
}
