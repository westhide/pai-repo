use crate::scanner::{helpers::is, Scanner};

impl<'s> Scanner<'s> {
    /// # Safety
    /// !BUG: if invoke in last line of file, will cause endless call stack
    #[inline]
    pub fn skip_ident_part(&mut self) {
        if let Some(f) = IDENT_PART_LOOKUP_TABLE[self.cur() as usize] {
            f(self)
        }
    }
}

type Handler = fn(&mut Scanner);

/// Ident Part
const IDENT_PART_LOOKUP_TABLE: &[Option<Handler>; 256] = &[
    // 0  1    2    3    4    5    6    7    8    9    A    B    C    D    E    F
    ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, // 0
    ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, // 1
    ___, ___, ___, ___, IDT, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, // 2
    IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, ___, ___, ___, ___, ___, ___, // 3
    ___, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, // 4
    IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, ___, BSH, ___, ___, IDT, // 5
    ___, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, // 6
    IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, ___, ___, ___, ___, ___, // 7
    ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, // 8
    ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, // 9
    ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, // A
    ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, // B
    ___, ___, UIP, UIP, UIP, UIP, UIP, UIP, UIP, UIP, UIP, UIP, UIP, UIP, UIP, UIP, // C
    UIP, UIP, UIP, UIP, UIP, UIP, UIP, UIP, UIP, UIP, UIP, UIP, UIP, UIP, UIP, UIP, // D
    UIP, UIP, UIP, UIP, UIP, UIP, UIP, UIP, UIP, UIP, UIP, UIP, UIP, UIP, UIP, UIP, // E
    UIP, UIP, UIP, UIP, UIP, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, // F
];

const ___: Option<Handler> = None;

/// ASCII Ident Part
/// - `0`..`9`
/// - `a`..`z`
/// - `A`..`Z`
/// - `$`
/// - `_`
const IDT: Option<Handler> = Some(|sn: &mut Scanner| {
    sn.skip(1);
    sn.skip_ident_part()
});

/// Unicode Ident Part
const UIP: Option<Handler> = Some(|sn: &mut Scanner| {
    let (ch, width) = sn.decode_char();

    if is::unicode_ident_part(ch) {
        sn.skip(width);
        sn.skip_ident_part()
    }
});

/// [UnicodeEscapeSequence][1] Ident part
/// - \u Hex4Digits
/// - \u {CodePoint}
///
/// [1]:https://tc39.es/ecma262/#prod-UnicodeEscapeSequence
const BSH: Option<Handler> = None;
