use std::{io, result};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    IO(#[from] io::Error),

    #[error("Lexer: {0}")]
    Lexer(String),
}

pub type Result<T> = result::Result<T, Error>;
