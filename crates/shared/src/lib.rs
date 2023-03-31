pub mod error;
pub mod macros;
pub mod unicode;

pub type Result<T> = std::result::Result<T, error::Error>;
