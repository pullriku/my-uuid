use crate::error::MyUuidError;

pub mod error;
pub mod random;
pub mod v4;

pub type Result<T> = std::result::Result<T, MyUuidError>;

pub const N_UUID_BYTES: usize = 16;
