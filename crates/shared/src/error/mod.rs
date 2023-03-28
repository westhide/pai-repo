use std::{io, result};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    IO(#[from] io::Error),
    #[error("Lexer: {0}")]
    Lexer(),
}

pub type Result<T> = result::Result<T, Error>;
