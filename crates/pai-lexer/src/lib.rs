#![feature(str_internals)]
#![feature(const_trait_impl)]
#![feature(ptr_sub_ptr)]
#![feature(slice_from_ptr_range)]
#![feature(test)]

extern crate core;
extern crate test;

use pai_shared::PResult;
use scanner::Scanner;

use crate::scanner::unit::Unit;

#[macro_use]
pub mod macros;

pub mod scanner;

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
            scanner: Scanner::new(src),
        }
    }
}

impl<'s> Iterator for Lexer<'s> {
    type Item = PResult<Unit<'s>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.scanner.next_unit()
    }
}
