#![feature(str_internals)]
#![feature(const_trait_impl)]
#![feature(ptr_sub_ptr)]
#![feature(slice_from_ptr_range)]
#![feature(test)]

extern crate core;
extern crate test;

use scanner::Scanner;
use token::Token;
use tokenize::Tokenize;

#[macro_use]
pub mod macros;

pub mod scanner;
pub mod token;
pub mod tokenize;

#[derive(Debug)]
pub struct Lexer<'s> {
    pub src: &'s str,
    pub scanner: Scanner<'s>,
}

impl<'s> Lexer<'s> {
    /// # Safety
    /// src must end with new line. e.g. LF(U+000A)
    pub fn new(src: &'s str) -> Self {
        Self {
            src,
            scanner: Scanner::new(src.as_bytes()),
        }
    }
}

impl<'s> Iterator for Lexer<'s> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

impl<'s> Tokenize<Lexer<'s>> for &'s str {
    fn tokenize(self) -> Lexer<'s> {
        Lexer::new(self)
    }
}
