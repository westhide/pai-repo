use pai_shared::unicode;

use crate::scanner::{
    comment::Comment,
    ident::Ident,
    keyword::{self, Keyword},
    punctuator::Punctuator,
    unit::Unit,
    Scanner,
};

pub type Entry = for<'s> fn(&mut Scanner<'s>) -> Unit<'s>;

#[inline]
pub fn lookup(index: u8) -> &'static Entry {
    &ENTRY_LOOKUP_TABLE[index as usize]
}

/// Byte entry lookup table
/// - [ASCII][1]
///
/// [1]:https://www.rfc-editor.org/rfc/rfc20
const ENTRY_LOOKUP_TABLE: &[Entry; 256] = &[
    // 0  1    2    3    4    5    6    7    8    9    A    B    C    D    E    F
    ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, // 0
    ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, // 1
    ___, EXL, QOT, HSH, DOL, PCT, APS, QOT, OPN, CPN, ATR, PLS, CMA, MIS, DOT, SLH, // 2
    ZRO, DIG, DIG, DIG, DIG, DIG, DIG, DIG, DIG, DIG, CLN, SMI, LST, EQL, GRT, QST, // 3
    ATS, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, // 4
    IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, OBT, BSH, CBT, CCF, UDL, // 5
    GAC, _A_, _B_, _C_, _D_, _E_, _F_, _G_, _H_, _I_, _J_, _K_, _L_, _M_, _N_, _O_, // 6
    _P_, _Q_, _R_, _S_, _T_, _U_, _V_, _W_, _X_, _Y_, _Y_, OBE, VLN, CBE, TID, ___, // 7
    UID, UID, UID, UID, UID, UID, UID, UID, UID, UID, UID, UID, UID, UID, UID, UID, // 8
    UID, UID, UID, UID, UID, UID, UID, UID, UID, UID, UID, UID, UID, UID, UID, UID, // 9
    UID, UID, UID, UID, UID, UID, UID, UID, UID, UID, UID, UID, UID, UID, UID, UID, // A
    UID, UID, UID, UID, UID, UID, UID, UID, UID, UID, UID, UID, UID, UID, UID, UID, // B
    UID, UID, UID, UID, UID, UID, UID, UID, UID, UID, UID, UID, UID, UID, UID, UID, // C
    UID, UID, UID, UID, UID, UID, UID, UID, UID, UID, UID, UID, UID, UID, UID, UID, // D
    UID, UID, UID, UID, UID, UID, UID, UID, UID, UID, UID, UID, UID, UID, UID, UID, // E
    UID, UID, UID, UID, UID, UID, UID, UID, UID, UID, UID, UID, UID, UID, UID, UID, // F
];

/// Error
const ___: Entry = |sn: &mut Scanner| {
    sn.skip(1);

    Unit::Err
};

#[inline]
fn scan_word<'s>(sn: &mut Scanner<'s>, skip_width: usize) -> &'s str {
    let start = sn.ptr;

    sn.skip(skip_width);

    sn.skip_ident_part();

    sn.sub_str(start..sn.ptr)
}

/// Ident
const IDT: Entry = |sn: &mut Scanner| unit!(Ident: scan_word(sn,1));

/// Exclamation
/// - `!`
const EXL: Entry = |sn: &mut Scanner| {
    sn.skip(1);

    if sn.eat(b'=') {
        if sn.eat(b'=') {
            unit!("!==")
        } else {
            unit!("!=")
        }
    } else {
        unit!("!")
    }
};

/// Quote
/// - `"`
/// - `'`
///
/// [String Literal](https://tc39.es/ecma262/#sec-literals-string-literals)
const QOT: Entry = |sn: &mut Scanner| {
    sn.skip(1);

    // scan string literal
    todo!()
};

/// Hash
/// - `#`
const HSH: Entry = |sn: &mut Scanner| {
    sn.skip(1);

    unit!("#")
};

/// Dollar
/// - `$`
const DOL: Entry = IDT;

/// Percentage
/// - `%`
const PCT: Entry = |sn: &mut Scanner| {
    sn.skip(1);

    if sn.eat(b'=') {
        unit!("%=")
    } else {
        unit!("%")
    }
};

/// Ampersand
/// - `&`
const APS: Entry = |sn: &mut Scanner| {
    sn.skip(1);

    if sn.eat(b'&') {
        if sn.eat(b'=') {
            unit!("&&=")
        } else {
            unit!("&&")
        }
    } else {
        if sn.eat(b'=') {
            unit!("&=")
        } else {
            unit!("&")
        }
    }
};

/// Opening parenthesis
/// - `(`
const OPN: Entry = |sn: &mut Scanner| {
    sn.skip(1);

    unit!("(")
};

/// Closing parenthesis
/// - `)`
const CPN: Entry = |sn: &mut Scanner| {
    sn.skip(1);

    unit!(")")
};

/// Asterisk
/// - `*`
const ATR: Entry = |sn: &mut Scanner| {
    sn.skip(1);

    if sn.eat(b'*') {
        if sn.eat(b'=') {
            unit!("**=")
        } else {
            unit!("**")
        }
    } else {
        if sn.eat(b'=') {
            unit!("*=")
        } else {
            unit!("*")
        }
    }
};

/// Plus
/// - `+`
const PLS: Entry = |sn: &mut Scanner| {
    sn.skip(1);

    if sn.eat(b'+') {
        unit!("++")
    } else {
        if sn.eat(b'=') {
            unit!("+=")
        } else {
            unit!("+")
        }
    }
};

/// Comma
/// - `,`
const CMA: Entry = |sn: &mut Scanner| {
    sn.skip(1);

    unit!(",")
};

/// Minus
/// - `-`
const MIS: Entry = |sn: &mut Scanner| {
    sn.skip(1);

    if sn.eat(b'-') {
        unit!("--")
    } else {
        if sn.eat(b'=') {
            unit!("-=")
        } else {
            unit!("-")
        }
    }
};

/// Dot
/// - `.`
const DOT: Entry = |sn: &mut Scanner| {
    sn.skip(1);

    if sn.eat(b'.') {
        if sn.eat(b'.') {
            unit!("...")
        } else {
            Unit::Err
        }
    } else {
        if sn.cur().is_ascii_digit() {
            // scan number
            todo!()
        } else {
            unit!(".")
        }
    }
};

/// Slash
/// - `/`
const SLH: Entry = |sn: &mut Scanner| {
    sn.skip(1);

    // scan line comment
    if sn.eat(b'/') {
        let start = sn.ptr;

        sn.skip_line();

        return Unit::Comment(Comment::Line(sn.sub_str(start..sn.ptr)));
    }

    // scan block comment
    if sn.eat(b'*') {
        return sn.scan_block_comment();
    }

    if sn.eat(b'=') {
        unit!("/=")
    } else {
        unit!("/")
    }
};

/// Zero
/// - `0`
const ZRO: Entry = |sn: &mut Scanner| {
    // scan number lit
    todo!()
};

/// Digit
/// - `1`..`9`
const DIG: Entry = |sn: &mut Scanner| {
    // scan number
    todo!()
};

/// Colon
/// - `:`
const CLN: Entry = |sn: &mut Scanner| {
    sn.skip(1);

    unit!(":")
};

/// Semicolon
/// - `;`
const SMI: Entry = |sn: &mut Scanner| {
    sn.skip(1);

    unit!(";")
};

/// Less than
/// - `<`
const LST: Entry = |sn: &mut Scanner| {
    sn.skip(1);

    if sn.eat(b'<') {
        if sn.eat(b'=') {
            unit!("<<=")
        } else {
            unit!("<<")
        }
    } else {
        if sn.eat(b'=') {
            unit!("<=")
        } else {
            unit!("<")
        }
    }
};

/// Equals
/// - `=`
const EQL: Entry = |sn: &mut Scanner| {
    sn.skip(1);

    if sn.eat(b'=') {
        if sn.eat(b'=') {
            unit!("===")
        } else {
            unit!("==")
        }
    } else {
        if sn.eat(b'>') {
            unit!("=>")
        } else {
            unit!("=")
        }
    }
};

/// Greater than
/// - `>`
const GRT: Entry = |sn: &mut Scanner| {
    sn.skip(1);

    if sn.eat(b'>') {
        if sn.eat(b'>') {
            if sn.eat(b'=') {
                unit!(">>>=")
            } else {
                unit!(">>>")
            }
        } else {
            if sn.eat(b'=') {
                unit!(">>=")
            } else {
                unit!(">>")
            }
        }
    } else {
        if sn.eat(b'=') {
            unit!(">=")
        } else {
            unit!(">")
        }
    }
};

/// Question
/// - `?`
const QST: Entry = |sn: &mut Scanner| {
    sn.skip(1);

    if sn.eat(b'?') {
        if sn.eat(b'=') {
            unit!("??=")
        } else {
            unit!("??")
        }
    } else {
        if sn.eat(b'.') {
            unit!("?.")
        } else {
            unit!("?")
        }
    }
};

/// At sign
/// - `@`
const ATS: Entry = |sn: &mut Scanner| {
    sn.skip(1);

    unit!("@")
};

/// Opening bracket
/// - `[`
const OBT: Entry = |sn: &mut Scanner| {
    sn.skip(1);

    unit!("[")
};

/// Backslash
/// - \
///
/// [UnicodeEscapeSequence](https://tc39.es/ecma262/#prod-UnicodeEscapeSequence)
const BSH: Entry = |sn: &mut Scanner| {
    sn.skip(1);

    // TODO: support unicode escape sequence
    // sn.skip_ident_part()
    Unit::Err
};

/// Closing bracket
/// - `]`
const CBT: Entry = |sn: &mut Scanner| {
    sn.skip(1);

    unit!("]")
};

/// Circumflex
/// - `^`
const CCF: Entry = |sn: &mut Scanner| {
    sn.skip(1);

    if sn.eat(b'=') {
        unit!("^=")
    } else {
        unit!("^")
    }
};

/// Underline
/// - `_`
const UDL: Entry = IDT;

/// Grave accent
/// - `
const GAC: Entry = |sn: &mut Scanner| {
    // template literal
    todo!()
};

/// Opening brace
/// - `{`
const OBE: Entry = |sn: &mut Scanner| {
    sn.skip(1);

    unit!("{")
};

/// Vertical line
/// - `|`
const VLN: Entry = |sn: &mut Scanner| {
    sn.skip(1);

    if sn.eat(b'|') {
        if sn.eat(b'=') {
            unit!("||=")
        } else {
            unit!("||")
        }
    } else {
        if sn.eat(b'=') {
            unit!("|=")
        } else {
            unit!("|")
        }
    }
};

/// Closing brace
/// - `}`
const CBE: Entry = |sn: &mut Scanner| {
    sn.skip(1);

    unit!("}")
};

/// Tilde
/// - `~`
const TID: Entry = |sn: &mut Scanner| {
    sn.skip(1);

    unit!("~")
};

/// Keyword or Ident
/// - `a`..`z`
const _A_: Entry = |sn: &mut Scanner| {
    match scan_word(sn, 1) {
        keyword::AWAIT => unit!("await"),
        ident => unit!(Ident: ident),
    }
};

const _B_: Entry = |sn: &mut Scanner| {
    match scan_word(sn, 1) {
        keyword::BREAK => unit!("break"),
        ident => unit!(Ident: ident),
    }
};

const _C_: Entry = |sn: &mut Scanner| {
    match scan_word(sn, 1) {
        keyword::CASE => unit!("case"),
        keyword::CATCH => unit!("catch"),
        keyword::CLASS => unit!("class"),
        keyword::CONST => unit!("const"),
        keyword::CONTINUE => unit!("continue"),
        ident => unit!(Ident: ident),
    }
};

const _D_: Entry = |sn: &mut Scanner| {
    match scan_word(sn, 1) {
        keyword::DEBUGGER => unit!("debugger"),
        keyword::DEFAULT => unit!("default"),
        keyword::DELETE => unit!("delete"),
        keyword::DO => unit!("do"),
        ident => unit!(Ident: ident),
    }
};

const _E_: Entry = |sn: &mut Scanner| {
    match scan_word(sn, 1) {
        keyword::ELSE => unit!("else"),
        keyword::ENUM => unit!("enum"),
        keyword::EXPORT => unit!("export"),
        keyword::EXTENDS => unit!("extends"),
        ident => unit!(Ident: ident),
    }
};

const _F_: Entry = |sn: &mut Scanner| {
    match scan_word(sn, 1) {
        keyword::FALSE => unit!("false"),
        keyword::FINALLY => unit!("finally"),
        keyword::FOR => unit!("for"),
        keyword::FUNCTION => unit!("function"),
        ident => unit!(Ident: ident),
    }
};

const _G_: Entry = IDT;

const _H_: Entry = IDT;

const _I_: Entry = |sn: &mut Scanner| {
    match scan_word(sn, 1) {
        keyword::IF => unit!("if"),
        keyword::IMPORT => unit!("import"),
        keyword::IN => unit!("in"),
        keyword::INSTANCEOF => unit!("instanceof"),
        ident => unit!(Ident: ident),
    }
};

const _J_: Entry = IDT;

const _K_: Entry = IDT;

const _L_: Entry = |sn: &mut Scanner| {
    match scan_word(sn, 1) {
        keyword::LET => unit!("let"),
        ident => unit!(Ident: ident),
    }
};

const _M_: Entry = IDT;

const _N_: Entry = |sn: &mut Scanner| {
    match scan_word(sn, 1) {
        keyword::NEW => unit!("new"),
        keyword::NULL => unit!("null"),
        ident => unit!(Ident: ident),
    }
};

const _O_: Entry = IDT;
const _P_: Entry = IDT;
const _Q_: Entry = IDT;

const _R_: Entry = |sn: &mut Scanner| {
    match scan_word(sn, 1) {
        keyword::RETURN => unit!("return"),
        ident => unit!(Ident: ident),
    }
};

const _S_: Entry = |sn: &mut Scanner| {
    match scan_word(sn, 1) {
        keyword::SUPER => unit!("super"),
        keyword::SWITCH => unit!("switch"),
        ident => unit!(Ident: ident),
    }
};

const _T_: Entry = |sn: &mut Scanner| {
    match scan_word(sn, 1) {
        keyword::THIS => unit!("this"),
        keyword::THROW => unit!("throw"),
        keyword::TRUE => unit!("true"),
        keyword::TRY => unit!("try"),
        keyword::TYPEOF => unit!("typeof"),
        ident => unit!(Ident: ident),
    }
};

const _U_: Entry = IDT;

const _V_: Entry = |sn: &mut Scanner| {
    match scan_word(sn, 1) {
        keyword::VAR => unit!("var"),
        keyword::VOID => unit!("void"),
        ident => unit!(Ident: ident),
    }
};

const _W_: Entry = |sn: &mut Scanner| {
    match scan_word(sn, 1) {
        keyword::WHILE => unit!("while"),
        keyword::WITH => unit!("with"),
        ident => unit!(Ident: ident),
    }
};

const _X_: Entry = IDT;

const _Y_: Entry = |sn: &mut Scanner| {
    match scan_word(sn, 1) {
        keyword::YIELD => unit!("yield"),
        ident => unit!(Ident: ident),
    }
};

const _Z_: Entry = IDT;

/// Unicode start Ident
const UID: Entry = |sn: &mut Scanner| {
    let (ch, width) = sn.decode_char();

    if unicode::is_ident_start(ch) {
        unit!(Ident: scan_word(sn, width))
    } else {
        sn.skip(width);

        Unit::Err
    }
};
