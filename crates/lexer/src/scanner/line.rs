use crate::scanner::Scanner;

#[inline]
pub fn is_unicode_line_terminator(ch: char) -> bool {
    matches!(ch, '\u{2028}' | '\u{2029}')
}

pub type Handler = fn(&mut Scanner);

#[inline]
pub fn lookup(index: u8) -> &'static Handler {
    &LINE_TERMINATOR_LOOKUP_TABLE[index as usize]
}

const LINE_TERMINATOR_LOOKUP_TABLE: &[Handler; 256] = &[
    // 0  1    2    3    4    5    6    7    8    9    A    B    C    D    E    F
    _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, NLN, _A_, _A_, NLN, _A_, _A_, // 0
    _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, // 1
    _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, // 2
    _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, // 3
    _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, // 4
    _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, // 5
    _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, // 6
    _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, // 7
    _U_, _U_, _U_, _U_, _U_, _U_, _U_, _U_, _U_, _U_, _U_, _U_, _U_, _U_, _U_, _U_, // 8
    _U_, _U_, _U_, _U_, _U_, _U_, _U_, _U_, _U_, _U_, _U_, _U_, _U_, _U_, _U_, _U_, // 9
    _U_, _U_, _U_, _U_, _U_, _U_, _U_, _U_, _U_, _U_, _U_, _U_, _U_, _U_, _U_, _U_, // A
    _U_, _U_, _U_, _U_, _U_, _U_, _U_, _U_, _U_, _U_, _U_, _U_, _U_, _U_, _U_, _U_, // B
    _U_, _U_, _U_, _U_, _U_, _U_, _U_, _U_, _U_, _U_, _U_, _U_, _U_, _U_, _U_, _U_, // C
    _U_, _U_, _U_, _U_, _U_, _U_, _U_, _U_, _U_, _U_, _U_, _U_, _U_, _U_, _U_, _U_, // D
    _U_, _U_, UE2, _U_, _U_, _U_, _U_, _U_, _U_, _U_, _U_, _U_, _U_, _U_, _U_, _U_, // E
    _U_, _U_, _U_, _U_, _U_, _U_, _U_, _U_, _U_, _U_, _U_, _U_, _U_, _U_, _U_, _U_, // F
];

const NLN: Handler = |sn| {};

// ASCII
const _A_: Handler = |sn| {
    sn.skip(1);
    sn.skip_line()
};

// Unicode
const _U_: Handler = |sn| {
    sn.skip_char();
    sn.skip_line()
};

/// Unicode line terminator
/// - `U+2028` : `[0xE2, 0x80, 0xA8]`
/// - `U+2029` : `[0xE2, 0x80, 0xA9]`
const UE2: Handler = |sn| {
    let (ch, width) = sn.decode_char();

    if !is_unicode_line_terminator(ch) {
        sn.skip(width);
        sn.skip_line()
    }
};
