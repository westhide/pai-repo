use crate::scanner::Scanner;

impl<'s> Scanner<'s> {
    /// Scan hashbang before iterate
    ///
    /// [Hashbang Comments](https://tc39.es/ecma262/#sec-hashbang)
    pub fn scan_hashbang(&mut self) -> Option<&'s str> {
        if self.cur() == b'#' && self.peek(1) == b'!' {
            self.skip(2);

            let start = self.ptr;

            self.skip_line();

            Some(self.sub_str(start..self.ptr))
        } else {
            None
        }
    }
}
