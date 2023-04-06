use crate::scanner::helpers::is::Unicode;

/// [ECMA IdentifierName][1]
///
/// [1]:https://tc39.es/ecma262/#sec-names-and-keywords
#[derive(Debug)]
pub struct Ident<'s> {
    pub raw: &'s str,
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
            Unicode::is_ident_start(self)
        }
    }

    fn is_ident_part(&self) -> bool {
        // ASCII chars are so common, check first
        if matches!(*self, 'a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '$') {
            true
        } else {
            Unicode::is_ident_part(self)
        }
    }
}
