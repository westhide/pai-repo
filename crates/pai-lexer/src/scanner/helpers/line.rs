use crate::scanner::{helpers::is::Unicode, Scanner};

impl<'s> Scanner<'s> {
    pub fn scan_line(&mut self) {
        LINE_TERMINATOR_LOOKUP_TABLE[self.byte() as usize](self)
    }
}

type Handler = fn(&mut Scanner);

const LINE_TERMINATOR_LOOKUP_TABLE: &[Handler; 256] = &[
    // 0  1    2    3    4    5    6    7    8    9    A    B    C    D    E    F
    EOF, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, NLN, _A_, _A_, NLN, _A_, _A_, // 0
    _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, // 1
    _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, // 2
    _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, // 3
    _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, // 4
    _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, // 5
    _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, // 6
    _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, _A_, // 7
    ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, // 8
    ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, // 9
    ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, // A
    ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, // B
    ___, ___, _U_, _U_, _U_, _U_, _U_, _U_, _U_, _U_, _U_, _U_, _U_, _U_, _U_, _U_, // C
    _U_, _U_, _U_, _U_, _U_, _U_, _U_, _U_, _U_, _U_, _U_, _U_, _U_, _U_, _U_, _U_, // D
    _U_, _U_, UE2, _U_, _U_, _U_, _U_, _U_, _U_, _U_, _U_, _U_, _U_, _U_, _U_, _U_, // E
    _U_, _U_, _U_, _U_, _U_, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, // F
];

/// Unreachable
const ___: Handler = |_| unreachable!("Invalid UTF8 lead byte");

/// End of file
const EOF: Handler = |_| {};

/// New line
const NLN: Handler = |_| {};

// ASCII
const _A_: Handler = |sn| {
    sn.skip(1);
    sn.scan_line()
};

// Unicode
const _U_: Handler = |sn| {
    sn.skip_char();
    sn.scan_line()
};

/// Unicode line terminator
/// - `U+2028` : `[0xE2, 0x80, 0xA8]`
/// - `U+2029` : `[0xE2, 0x80, 0xA9]`
const UE2: Handler = |sn| {
    if !sn.char().is_line_terminator() {
        sn.skip_char();
        sn.scan_line()
    }
};
