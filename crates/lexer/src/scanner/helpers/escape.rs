use pai_shared::{err, PResult};

use crate::scanner::{
    helpers::is::{Digit, Radix::Hex},
    Scanner,
};

impl<'s> Scanner<'s> {
    /// [UnicodeEscapeSequence](https://tc39.es/ecma262/#prod-UnicodeEscapeSequence)
    pub fn scan_escape_unicode(&mut self) -> PResult<char> {
        if self.eat(b'{') {
            // [CodePoint](https://tc39.es/ecma262/#prod-CodePoint)
            self.mark();

            while !self.is_empty() {
                if self.byte() == b'}' {
                    break;
                }

                if self.byte().is_digit(Hex) {
                    self.skip(1)
                } else {
                    return err!("Invalid escape HexDigit");
                }
            }

            self.down();
            self.skip(1);

            let code_point = u32::from_str_radix(self.raw(), 16)?;

            if code_point <= 0x10FFFF {
                Ok(char::try_from(code_point)?)
            } else {
                err!("Invalid escape CodePoint")
            }
        } else {
            self.mark();

            // [Hex4Digits](https://tc39.es/ecma262/#prod-Hex4Digits)
            for _ in 0..4 {
                if self.byte().is_digit(Hex) {
                    self.skip(1)
                } else {
                    return err!("Invalid escape HexDigit");
                }
            }

            self.down();

            Ok(u32::from_str_radix(self.raw(), 16)?.try_into()?)
        }
    }
}
