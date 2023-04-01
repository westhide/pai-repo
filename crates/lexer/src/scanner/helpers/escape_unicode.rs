use pai_shared::{err, PResult};

use crate::scanner::Scanner;

impl<'s> Scanner<'s> {
    /// [UnicodeEscapeSequence](https://tc39.es/ecma262/#prod-UnicodeEscapeSequence)
    pub fn scan_escape_unicode(&mut self) -> PResult<char> {
        if self.eat(b'{') {
            // [CodePoint](https://tc39.es/ecma262/#prod-CodePoint)

            let start = self.ptr;

            // !BUG: if CodePoint unterminated, may cause endless loop
            while self.cur() != b'}' {
                if self.cur().is_ascii_hexdigit() {
                    self.skip(1)
                } else {
                    return err!("Invalid escape unicode HexDigit '{}'", self.cur_char());
                }
            }

            let code_point = self.sub_str(start..self.ptr);

            let value = u32::from_str_radix(code_point, 16)?;

            if value > 0x10FFFF {
                return err!("Invalid escape unicode CodePoint '{code_point}'");
            }

            self.skip(1);

            Ok(char::try_from(value)?)
        } else {
            let start = self.ptr;

            // [Hex4Digits](https://tc39.es/ecma262/#prod-Hex4Digits)
            for _ in 0..4 {
                if self.cur().is_ascii_hexdigit() {
                    self.skip(1)
                } else {
                    return err!("Invalid escape unicode HexDigit '{}'", self.cur_char());
                }
            }

            let code_point = self.sub_str(start..self.ptr);

            Ok(u32::from_str_radix(code_point, 16)?.try_into()?)
        }
    }
}
