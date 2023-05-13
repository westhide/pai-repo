pub mod error;
pub mod macros;
pub mod unicode;

pub type PResult<T> = Result<T, error::PError>;
