use pai_shared::unicode;

use crate::scanner::Scanner;

/// [ECMA IdentifierName][1]
///
/// [1]:https://tc39.es/ecma262/#sec-names-and-keywords
#[derive(Debug)]
pub struct Ident<'s> {
    raw: &'s str,
}

impl<'s> Ident<'s> {
    pub fn new(s: &'s str) -> Self {
        Self { raw: s }
    }
}

pub trait Identifier {
    fn is_ident_start(&self) -> bool;

    fn is_ident_part(&self) -> bool;
}

impl Identifier for char {
    fn is_ident_start(&self) -> bool {
        // ASCII chars are so common, check first
        if matches!(*self, 'a'..='z' | 'A'..='Z' | '_' | '$') {
            true
        } else {
            unicode::is_ident_start(*self)
        }
    }

    fn is_ident_part(&self) -> bool {
        // ASCII chars are so common, check first
        if matches!(*self, 'a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '$') {
            true
        } else {
            matches!(*self, '\u{200C}' | '\u{200D}') || unicode::is_ident_continue(*self)
        }
    }
}

pub type Handler = fn(&mut Scanner);

#[inline]
pub fn lookup(index: u8) -> &'static Option<Handler> {
    &IDENT_PART_LOOKUP_TABLE[index as usize]
}

/// Ident Part
const IDENT_PART_LOOKUP_TABLE: &[Option<Handler>; 256] = &[
    // 0  1    2    3    4    5    6    7    8    9    A    B    C    D    E    F
    ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, // 0
    ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, // 1
    ___, ___, ___, ___, IDT, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, // 2
    ___, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, ___, ___, ___, ___, ___, ___, // 3
    ___, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, // 4
    IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, ___, BSH, ___, ___, IDT, // 5
    ___, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, // 6
    IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, ___, ___, ___, ___, ___, // 7
    UIP, UIP, UIP, UIP, UIP, UIP, UIP, UIP, UIP, UIP, UIP, UIP, UIP, UIP, UIP, UIP, // 8
    UIP, UIP, UIP, UIP, UIP, UIP, UIP, UIP, UIP, UIP, UIP, UIP, UIP, UIP, UIP, UIP, // 9
    UIP, UIP, UIP, UIP, UIP, UIP, UIP, UIP, UIP, UIP, UIP, UIP, UIP, UIP, UIP, UIP, // A
    UIP, UIP, UIP, UIP, UIP, UIP, UIP, UIP, UIP, UIP, UIP, UIP, UIP, UIP, UIP, UIP, // B
    UIP, UIP, UIP, UIP, UIP, UIP, UIP, UIP, UIP, UIP, UIP, UIP, UIP, UIP, UIP, UIP, // C
    UIP, UIP, UIP, UIP, UIP, UIP, UIP, UIP, UIP, UIP, UIP, UIP, UIP, UIP, UIP, UIP, // D
    UIP, UIP, UIP, UIP, UIP, UIP, UIP, UIP, UIP, UIP, UIP, UIP, UIP, UIP, UIP, UIP, // E
    UIP, UIP, UIP, UIP, UIP, UIP, UIP, UIP, UIP, UIP, UIP, UIP, UIP, UIP, UIP, UIP, // F
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

    if matches!(ch, '\u{200C}' | '\u{200D}') || unicode::is_ident_continue(ch) {
        sn.skip(width);
        sn.skip_ident_part()
    }
});

/// [UnicodeEscapeSequence][1] Ident
/// - \u Hex4Digits
/// - \u {CodePoint}
///
/// [1]:https://tc39.es/ecma262/#prod-UnicodeEscapeSequence
const BSH: Option<Handler> = Some(|sn: &mut Scanner| {
    // !Non-support
    // unicode escape sequence
    sn.skip(1);
    // sn.skip_ident_part()
});
