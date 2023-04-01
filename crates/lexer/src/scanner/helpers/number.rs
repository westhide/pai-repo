use pai_shared::{err, PResult};

use crate::scanner::Scanner;

impl<'s> Scanner<'s> {
    /// [DecimalDigit](https://tc39.es/ecma262/#prod-DecimalDigit)
    pub fn scan_decimal<const DOT: bool>(&mut self) -> PResult<bool> {
        let mut decimal_point = DOT;
        let mut exponent_part = true;

        loop {
            match self.cur() {
                b'0'..=b'9' => self.skip(1),
                b'_' => {
                    if unsafe { *self.ptr.offset(-1) }.is_ascii_digit() {
                        self.skip(1)
                    } else {
                        return err!("Invalid Numeric separator");
                    }
                },
                b'.' if decimal_point => {
                    decimal_point = false;
                    self.skip(1)
                },
                b'e' | b'E' if exponent_part => {
                    if matches!(self.cur(), b'+' | b'-') {
                        self.skip(1)
                    }

                    if self.cur().is_ascii_digit() {
                        self.skip(1)
                    } else {
                        return err!("Invalid DecimalLiteral ExponentPart");
                    }
                },
                b'n' if decimal_point && exponent_part => {
                    self.skip(1);
                    return Ok(true);
                },
                _ => return Ok(false),
            }
        }
    }

    /// [NonDecimalIntegerLiteral](https://tc39.es/ecma262/#prod-NonDecimalIntegerLiteral)
    /// - Binary: `0[bB]` [0-1] _? [0-1] n?
    /// - Octal: `0[oO]` [0-7] _? [0-7] n?
    /// - Hex: `0[xX]` [0-F] _? [0-F] n?
    pub fn scan_radix_int(&mut self, radix: u8) -> PResult<bool> {
        if self.is_digit(radix) {
            self.skip(1)
        } else {
            return err!("Invalid {radix} radix digit '{}'", self.cur_char());
        }

        loop {
            if self.is_digit(radix) {
                self.skip(1)
            } else {
                if self.eat(b'_') {
                    if self.is_digit(radix) {
                        self.skip(1);
                    } else {
                        return err!("Invalid Numeric separator");
                    }
                } else {
                    return Ok(self.eat(b'n'));
                }
            }
        }
    }
}
