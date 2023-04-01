use crate::scanner::{helpers::is, Scanner};

impl<'s> Scanner<'s> {
    /// # Safety
    /// !BUG: if invoke in last line of file, will cause endless call stack
    #[inline]
    pub fn skip_line(&mut self) {
        LINE_TERMINATOR_LOOKUP_TABLE[self.cur() as usize](self)
    }
}

type Handler = fn(&mut Scanner);

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
const ___: Handler = |_| unreachable!("Invalid UTF8");

/// New line
const NLN: Handler = |_| {};

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

    if !is::unicode_line_terminator(ch) {
        sn.skip(width);
        sn.skip_line()
    }
};
