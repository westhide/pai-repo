use pai_shared::PResult;

use crate::scanner::{lit::Lit, unit::Unit, Scanner};

impl<'s> Scanner<'s> {
    /// [String Literal](https://tc39.es/ecma262/#sec-literals-string-literals)
    pub fn scan_string(&mut self, quote: u8) -> PResult<Unit<'s>> {
        self.skip(1);
        self.mark();

        // !BUG
        // String Literals unchecked
        while self.byte() != quote {
            if self.byte() == b'\\' {
                self.skip(1)
            }

            self.skip_char()
        }

        self.down();
        self.skip(1);

        Ok(unit!(String: self.raw()))
    }
}
