use pai_shared::unicode;

#[inline]
pub fn unicode_space(ch: char) -> bool {
    matches! {
        ch,
        '\u{00A0}'              |
        '\u{1680}'              |
        '\u{2000}'..='\u{200A}' |
        '\u{202F}'              |
        '\u{205F}'              |
        '\u{3000}'              |
        '\u{FEFF}'
    }
}

#[inline]
pub fn unicode_line_terminator(ch: char) -> bool {
    matches!(ch, '\u{2028}' | '\u{2029}')
}

#[inline]
pub fn unicode_ident_part(ch: char) -> bool {
    unicode::is_ident_continue(ch) || matches!(ch, '\u{200C}' | '\u{200D}')
}
