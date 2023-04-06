use kind::Kind;
use span::Span;

mod kind;
mod span;

#[derive(Debug)]
pub struct Token {
    kind: Kind,
    span: Span,
}
