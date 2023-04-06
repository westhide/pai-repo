use std::{marker::PhantomData, slice};

use pai_shared::PResult;

use crate::scanner::unit::Unit;

pub mod comment;
#[allow(clippy::collapsible_else_if)]
pub mod entry;
pub mod helpers;
pub mod ident;
pub mod keyword;
pub mod lit;
pub mod punctuator;
pub mod unit;

/// High performance u8 slice scanner, Inspired by [slice::Iter]
///
/// # Safety
#[derive(Debug)]
pub struct Scanner<'s> {
    ptr: *const u8,
    end: *const u8,

    lo: *const u8,
    hi: *const u8,

    _marker: PhantomData<&'s u8>,
}

impl<'s> Scanner<'s> {
    pub fn new(s: &'s str) -> Self {
        // SAFETY: &str guaranteed ptr and end is valid
        unsafe {
            let ptr = s.as_ptr();
            let end = ptr.add(s.len());

            Self {
                ptr,
                end,
                lo: ptr,
                hi: ptr,
                _marker: PhantomData,
            }
        }
    }

    #[inline]
    pub fn byte(&self) -> u8 {
        unsafe { *self.ptr }
    }

    #[inline]
    pub fn peek(&self, count: isize) -> u8 {
        // !FIX
        // remove this
        #[cfg(feature = "safe-scanner-peek")]
        assert!(count <= self.len() as isize);

        unsafe { *self.ptr.offset(count) }
    }

    #[inline]
    pub fn skip(&mut self, count: usize) {
        #[cfg(feature = "safe-scanner-skip")]
        assert!(count <= self.len());

        unsafe { self.ptr = self.ptr.add(count) }
    }

    #[inline]
    pub fn eat(&mut self, byte: u8) -> bool {
        // !FIX
        // remove this
        #[cfg(feature = "safe-scanner-eat")]
        if self.is_empty() {
            return false;
        }

        if self.byte() == byte {
            self.skip(1);
            true
        } else {
            false
        }
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
    #[inline]
    pub fn mark(&mut self) {
        self.lo = self.ptr
    }

    #[inline]
    pub fn down(&mut self) {
        self.hi = self.ptr
    }

    #[inline]
    pub fn raw(&self) -> &'s str {
        unsafe { std::str::from_utf8_unchecked(slice::from_ptr_range(self.lo..self.hi)) }
    }
}

impl<'s> Scanner<'s> {
    pub fn next_unit(&mut self) -> Option<PResult<Unit<'s>>> {
        self.skip_space();

        if self.is_empty() {
            None
        } else {
            Some(entry::lookup(self.byte())(self))
        }
    }
}
