use pai_shared::unicode;

pub trait Unicode {
    fn is_space(&self) -> bool;

    fn is_line_terminator(&self) -> bool;

    fn is_ident_start(&self) -> bool;

    fn is_ident_part(&self) -> bool;
}

impl Unicode for char {
    fn is_space(&self) -> bool {
        matches! {
            self,
            '\u{00A0}'              |
            '\u{1680}'              |
            '\u{2000}'..='\u{200A}' |
            '\u{202F}'              |
            '\u{205F}'              |
            '\u{3000}'              |
            '\u{FEFF}'
        }
    }

    fn is_line_terminator(&self) -> bool {
        matches!(self, '\u{2028}' | '\u{2029}')
    }

    fn is_ident_start(&self) -> bool {
        unicode::is_ident_start(*self)
    }

    fn is_ident_part(&self) -> bool {
        unicode::is_ident_continue(*self) || matches!(self, '\u{200C}' | '\u{200D}')
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Radix {
    Bin,
    Oct,
    Hex,
}

pub trait Digit {
    fn is_digit(&self, radix: Radix) -> bool;
}

impl Digit for u8 {
    fn is_digit(&self, radix: Radix) -> bool {
        match radix {
            Radix::Bin => matches!(self, b'0' | b'1'),
            Radix::Oct => matches!(self, b'0'..=b'7'),
            Radix::Hex => HEX_LOOKUP_TABLE[*self as usize],
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
