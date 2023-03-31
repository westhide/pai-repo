use crate::scanner::Scanner;

impl<'s> Scanner<'s> {
    /// [UTF8](https://tools.ietf.org/html/rfc3629)
    ///
    /// Inspired by [core::str::next_code_point]
    #[inline]
    pub fn decode_char(&self) -> (char, usize) {
        // | Char. number range      | UTF-8 octet sequence                |
        // |-------------------------|-------------------------------------|
        // | Hex Unicode             | Binary UTF8 byte                    |
        // | U+0000...U+007F         | 0xxxxxxx                            |
        // | U+0080...U+07FF         | 110xxxxx 10yyyyyy                   |
        // | U+0800...U+FFFF         | 1110xxxx 10yyyyyy 10zzzzzz          |
        // | U+10000...U+10FFFF      | 11110xxx 10yyyyyy 10zzzzzz 10wwwwww |

        let x = self.cur() as u32;

        // Ascii
        if x <= 0x7F {
            let ch = char!(x);

            return (ch, 1);
        }

        let y = self.peek(1) as u32;

        // UTF8-2
        if x < 0xE0 {
            let ch = char!((x & 0x1F) << 6 | y);

            return (ch, 2);
        }

        let z = self.peek(2) as u32;

        // UTF8-3
        if x < 0xF0 {
            let ch = char!((x & 0xF) << 12 | y << 6 | z);

            return (ch, 3);
        }

        // UTF8-4
        let w = self.peek(3) as u32;

        let ch = char!((x & 0x7) << 18 | y << 12 | z << 6 | w);

        (ch, 4)
    }

    #[inline]
    pub fn cur_char(&self) -> char {
        self.decode_char().0
    }

    #[inline]
    pub fn cur_ascii(&self) -> Option<char> {
        if self.cur().is_ascii() {
            Some(char!(self.cur() as u32))
        } else {
            None
        }
    }

    #[inline]
    pub fn skip_char(&mut self) {
        self.skip(core::str::utf8_char_width(self.cur()))
    }
}
