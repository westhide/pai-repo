use crate::scanner::{comment::Comment, ident::Ident, keyword::Keyword, lit::Lit, punc::Punc};

#[derive(Debug)]
pub enum Unit<'s> {
    Keyword(Keyword),

    Ident(Ident<'s>),

    /// Punctuator
    Punc(Punc),

    /// literal
    Lit(Lit<'s>),

    Comment(Comment<'s>),

    Err,
}
