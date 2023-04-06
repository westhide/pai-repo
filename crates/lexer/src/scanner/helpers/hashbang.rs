use crate::scanner::Scanner;

impl<'s> Scanner<'s> {
    /// Scan hashbang before iterate
    ///
    /// [Hashbang Comments](https://tc39.es/ecma262/#sec-hashbang)
    pub fn scan_hashbang(&mut self) -> Option<&'s str> {
        if self.byte() == b'#' && self.peek(1) == b'!' {
            self.skip(2);

            self.mark();
            self.scan_line();
            self.down();

            Some(self.raw())
        } else {
            None
        }
    }
}
