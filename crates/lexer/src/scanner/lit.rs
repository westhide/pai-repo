#[derive(Debug)]
pub enum Lit<'s> {
    String(&'s str),
    Number(&'s str),
}
