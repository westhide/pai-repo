use std::{marker::PhantomData, ops::Range, slice};

use pai_shared::PResult;

use crate::scanner::{comment::Comment, unit::Unit};

pub mod comment;
#[allow(clippy::collapsible_else_if)]
pub mod entry;
pub mod helpers;
pub mod ident;
pub mod keyword;
pub mod lit;
pub mod punctuator;
pub mod unit;

/// High performance u8 slice scanner, Inspired by from [slice::Iter]
///
/// # Safety
#[derive(Debug)]
pub struct Scanner<'s> {
    ptr: *const u8,
    end: *const u8,
    _marker: PhantomData<&'s u8>,
}

impl<'s> Scanner<'s> {
    pub fn new(slice: &'s [u8]) -> Self {
        // SAFETY: slice guaranteed ptr and end is valid
        unsafe {
            let ptr = slice.as_ptr();
            let end = ptr.add(slice.len());

            Self {
                ptr,
                end,
                _marker: PhantomData,
            }
        }
    }

    #[inline]
    pub fn cur(&self) -> u8 {
        unsafe { *self.ptr }
    }

    #[inline]
    pub fn peek(&self, count: usize) -> u8 {
        #[cfg(feature = "safe-scanner-peek")]
        assert!(count <= self.len());

        unsafe { *self.ptr.add(count) }
    }

    #[inline]
    pub fn skip(&mut self, count: usize) {
        #[cfg(feature = "safe-scanner-skip")]
        assert!(count <= self.len());

        unsafe { self.ptr = self.ptr.add(count) };
    }

    #[inline]
    pub fn eat(&mut self, byte: u8) -> bool {
        #[cfg(feature = "safe-scanner-eat")]
        if self.is_empty() {
            return false;
        }

        if self.cur() == byte {
            self.skip(1);
            true
        } else {
            false
        }
    }

    #[inline]
    pub fn sub_str(&self, range: Range<*const u8>) -> &'s str {
        unsafe { std::str::from_utf8_unchecked(slice::from_ptr_range(range)) }
    }

    #[inline]
    pub fn len(&self) -> usize {
        unsafe { self.end.sub_ptr(self.ptr) }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.ptr == self.end
    }
}

impl<'s> Scanner<'s> {
    /// # Safety
    /// !BUG: if block comment unterminated, will cause endless loop
    #[inline]
    pub fn scan_block_comment(&mut self) -> Unit<'s> {
        let start = self.ptr;

        loop {
            if self.eat(b'*') && self.eat(b'/') {
                let end = unsafe { self.ptr.offset(-2) };

                return unit!(BlockComment: self.sub_str(start..end));
            }

            self.skip_char()
        }
    }
}

impl<'s> Scanner<'s> {
    pub fn next(&mut self) -> Option<PResult<Unit>> {
        self.skip_space();

        if self.is_empty() {
            None
        } else {
            let f = entry::lookup(self.cur());

            Some(f(self))
        }
    }
}
