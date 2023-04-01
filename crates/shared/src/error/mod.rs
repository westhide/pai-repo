use std::{char, io, num};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum PError {
    #[error(transparent)]
    IO(#[from] io::Error),

    #[error(transparent)]
    ParseInt(#[from] num::ParseIntError),

    #[error(transparent)]
    ParseFloat(#[from] num::ParseFloatError),

    #[error(transparent)]
    CharTryFrom(#[from] char::CharTryFromError),

    #[error("{0}")]
    Info(String),
}
