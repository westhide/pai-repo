use pai_shared::{err, PResult};

use crate::scanner::{
    helpers::is::{Digit, Radix},
    Scanner,
};

impl<'s> Scanner<'s> {
    /// [DecimalDigit](https://tc39.es/ecma262/#prod-DecimalDigit)

    pub fn scan_decimal(&mut self, allow_dot: bool) -> PResult<bool> {
        let mut decimal_point = allow_dot;
        let mut exponent_part = true;

        loop {
            match self.byte() {
                b'0'..=b'9' => self.skip(1),
                b'_' => {
                    if self.peek(-1).is_ascii_digit() && self.peek(1).is_ascii_digit() {
                        self.skip(2)
                    } else {
                        return err!("Invalid Numeric separator")
                    }
                },
                b'.' if decimal_point => {
                    decimal_point = false;
                    self.skip(1)
                },
                b'e' | b'E' if exponent_part => {
                    exponent_part = false;
                    self.skip(1);

                    if matches!(self.byte(), b'+' | b'-') {
                        self.skip(1)
                    }

                    if self.byte().is_ascii_digit() {
                        self.skip(1)
                    } else {
                        return err!("Invalid DecimalLiteral ExponentPart")
                    }
                },
                b'n' if decimal_point && exponent_part => {
                    self.skip(1);
                    return Ok(true)
                },
                _ => return Ok(false),
            }
        }
    }

    /// [NonDecimalIntegerLiteral](https://tc39.es/ecma262/#prod-NonDecimalIntegerLiteral)
    /// - Binary: `0[bB]` [0-1] _? [0-1] n?
    /// - Octal: `0[oO]` [0-7] _? [0-7] n?
    /// - Hex: `0[xX]` [0-F] _? [0-F] n?

    pub fn scan_radix_int(&mut self, radix: Radix) -> PResult<bool> {
        loop {
            if self.byte().is_digit(radix) {
                self.skip(1);
                continue
            }

            if self.byte() == b'_' {
                if self.peek(-1).is_digit(radix) && self.peek(1).is_digit(radix) {
                    self.skip(2);
                } else {
                    return err!("Invalid Numeric separator")
                }
            } else {
                return Ok(self.eat(b'n'))
            }
        }
    }
}
