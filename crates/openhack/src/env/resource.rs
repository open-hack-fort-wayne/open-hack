use std::fmt::Debug;

pub mod database;
pub mod password_manager;

pub trait Stable: Send + Sync + Debug {}
impl<T: Send + Sync + Debug> Stable for T {}

pub trait Resource<T>: Stable {
    fn as_res(&self) -> &T;
}
