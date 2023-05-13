use crate::scanner::{helpers::is::Unicode, Scanner};

impl<'s> Scanner<'s> {
    pub fn skip_space(&mut self) {
        if let Some(handler) = WHITESPACE_LOOKUP_TABLE[self.byte() as usize] {
            handler(self)
        }
    }
}

type Handler = fn(&mut Scanner);

/// Whitespace
///
/// [Unicode Whitespace #Zs](https://www.unicode.org/Public/UCD/latest/ucd/PropList.txt)
const WHITESPACE_LOOKUP_TABLE: &[Option<Handler>; 256] = &[
    // 0  1    2    3    4    5    6    7    8    9    A    B    C    D    E    F
    EOF, ___, ___, ___, ___, ___, ___, ___, ___, SPC, NLN, SPC, SPC, NLN, ___, ___, // 0
    ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, // 1
    SPC, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, // 2
    ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, // 3
    ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, // 4
    ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, // 5
    ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, // 6
    ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, // 7
    ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, // 8
    ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, // 9
    ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, // A
    ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, // B
    ___, ___, UWS, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, // C
    ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, // D
    ___, UWS, UWS, UWS, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, UWS, // E
    ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, // F
];

const ___: Option<Handler> = None;

/// End of file
const EOF: Option<Handler> = Some(|sn: &mut Scanner| sn.skip(1));

/// [Space](https://tc39.es/ecma262/#sec-white-space)
/// - `U+0009`
/// - `U+000B`
/// - `U+000C`
/// - `U+0020`
const SPC: Option<Handler> = Some(|sn: &mut Scanner| {
    sn.skip(1);
    sn.skip_space()
});

/// [Line terminator](https://tc39.es/ecma262/#sec-line-terminators)
/// - `U+000A`
/// - `U+000D`
const NLN: Option<Handler> = Some(|sn: &mut Scanner| {
    sn.skip(1);
    sn.skip_space()
});

/// Unicode Whitespace
const UWS: Option<Handler> = Some(|sn: &mut Scanner| {
    if sn.char().is_space() || sn.char().is_line_terminator() {
        sn.skip_char();
        sn.skip_space()
    }
});

// /// `U+00A0` : `[0xC2, 0xA0]`
// const UC2: Option<Handler> = Some(|sn: &mut Scanner| {
//     if unsafe { *sn.ptr.add(1) } == 0xA0 {
//         sn.skip(2);
//         sn.skip_space()
//     }
// });
//
// /// `U+1680` : `[0xE1, 0x9A, 0x80]`
// const UE1: Option<Handler> = Some(|sn: &mut Scanner| {
//     unsafe {
//         // SAFETY: mid point to the middle byte of [0xE1, 0x9A, 0x80]
//         let mid = sn.ptr.add(1) as *const u16;
//
//         // u16 byte order: Little-Endian
//         if *mid == 0x809A {
//             sn.skip(3);
//             sn.skip_space()
//         }
//     }
// });
//
// /// `U+2000`...`U+200A`
// /// - `[0xE2, 0x80, 0x80...0x8A]`
// ///
// /// `U+2028` : `[0xE2, 0x80, 0xA8]`
// ///
// /// `U+2029` : `[0xE2, 0x80, 0xA9]`
// ///
// /// `U+202F` : `[0xE2, 0x80, 0xAF]`
// ///
// /// `U+205F` : `[0xE2, 0x81, 0x9F]`
// const UE2: Option<Handler> = Some(|sn: &mut Scanner| {
//     if unsafe { *sn.ptr.add(1) } == 0x80 {
//         match unsafe { *sn.ptr.add(2) } {
//             0x80..=0x8A | 0xAF => {
//                 sn.skip(3);
//                 sn.skip_space()
//             },
//             0xA8 | 0xA9 => {
//                 // sn.new_line = true;
//                 sn.skip(3);
//                 sn.skip_space()
//             },
//             _ => {},
//         }
//     } else {
//         unsafe {
//             let mid = sn.ptr.add(1) as *const u16;
//
//             if *mid == 0x9F81 {
//                 sn.skip(3);
//                 sn.skip_space()
//             }
//         }
//     }
// });
//
// /// `U+3000` : `[0xE3, 0x80, 0x80]`
// const UE3: Option<Handler> = Some(|sn: &mut Scanner| {
//     unsafe {
//         let mid = sn.ptr.add(1) as *const u16;
//
//         if *mid == 0x8080 {
//             sn.skip(3);
//             sn.skip_space()
//         }
//     }
// });
//
// /// `U+FEFF` : `[0xEF, 0xBB, 0xBF]`
// const UEF: Option<Handler> = Some(|sn: &mut Scanner| {
//     unsafe {
//         let mid = sn.ptr.add(1) as *const u16;
//
//         if *mid == 0xBFBB {
//             sn.skip(3);
//             sn.skip_space()
//         }
//     }
// });
