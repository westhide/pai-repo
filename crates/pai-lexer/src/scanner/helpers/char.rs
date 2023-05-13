use crate::scanner::Scanner;

impl<'s> Scanner<'s> {
    /// [UTF8](https://tools.ietf.org/html/rfc3629)
    ///
    /// Inspired by [core::str::next_code_point]

    pub fn char(&self) -> char {
        // | Char. number range      | UTF-8 octet sequence                |
        // |-------------------------|-------------------------------------|
        // | Hex Unicode             | Binary UTF8 byte                    |
        // | U+0000...U+007F         | 0xxxxxxx                            |
        // | U+0080...U+07FF         | 110xxxxx 10yyyyyy                   |
        // | U+0800...U+FFFF         | 1110xxxx 10yyyyyy 10zzzzzz          |
        // | U+10000...U+10FFFF      | 11110xxx 10yyyyyy 10zzzzzz 10wwwwww |

        let x = self.byte() as u32;

        // Ascii
        if x < 0x80 {
            return char!(x)
        }

        let y = self.peek(1) as u32;

        // UTF8-2
        if x < 0xE0 {
            return char!((x & 0x1F) << 6 | y)
        }

        let z = self.peek(2) as u32;

        // UTF8-3
        if x < 0xF0 {
            return char!((x & 0xF) << 12 | y << 6 | z)
        }

        // UTF8-4
        let w = self.peek(3) as u32;

        char!((x & 0x7) << 18 | y << 12 | z << 6 | w)
    }

    pub fn skip_char(&mut self) {
        self.skip(core::str::utf8_char_width(self.byte()))
    }
}
