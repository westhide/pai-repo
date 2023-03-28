use crate::scanner::{
    comment::Comment, ident::Ident, keyword::Keyword, lit::Lit, punctuator::Punctuator,
};

#[derive(Debug)]
pub enum Unit<'s> {
    Keyword(Keyword),

    Ident(Ident<'s>),

    /// Punctuator
    Punctuator(Punctuator),

    /// literal
    Lit(Lit<'s>),

    Comment(Comment<'s>),

    Err,
}
