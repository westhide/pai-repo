use crate::scanner::Scanner;

impl<'s> Scanner<'s> {
    #[inline]
    pub fn is_binary_digit(&self) -> bool {
        matches!(self.cur(), b'0'..=b'1')
    }

    #[inline]
    pub fn is_octal_digit(&self) -> bool {
        matches!(self.cur(), b'0'..=b'7')
    }

    #[inline]
    pub fn is_hex_digit(&self) -> bool {
        HEX_LOOKUP_TABLE[self.cur() as usize]
    }

    #[inline]
    pub fn is_digit(&self, radix: u8) -> bool {
        match radix {
            2 => self.is_binary_digit(),
            8 => self.is_octal_digit(),
            16 => self.is_hex_digit(),
            radix => unreachable!("Invalid Digit radix: {}", radix),
        }
    }
}

const T: bool = true;
const F: bool = false;

const HEX_LOOKUP_TABLE: &[bool; 256] = &[
    // 1  2  3  4  5  6  7  8  9  A  B  C  D  E  F
    F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, // 0
    F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, // 1
    F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, // 2
    T, T, T, T, T, T, T, T, T, T, F, F, F, F, F, F, // 3
    F, T, T, T, T, T, T, F, F, F, F, F, F, F, F, F, // 4
    F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, // 5
    F, T, T, T, T, T, T, F, F, F, F, F, F, F, F, F, // 6
    F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, // 7
    F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, // 8
    F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, // 9
    F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, // A
    F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, // B
    F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, // C
    F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, // D
    F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, // E
    F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, // F
];
