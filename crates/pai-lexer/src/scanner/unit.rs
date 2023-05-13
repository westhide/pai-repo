use crate::scanner::{
    comment::Comment, ident::Ident, keyword::Keyword, lit::Lit, punctuator::Punctuator,
};

#[derive(Debug)]
pub enum Unit<'s> {
    Keyword(Keyword),

    /// Punctuator
    Punctuator(Punctuator),

    Ident(Ident<'s>),

    /// literal
    Lit(Lit<'s>),

    Comment(Comment<'s>),
}
