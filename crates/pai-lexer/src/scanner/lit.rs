#[derive(Debug)]
pub enum Lit<'s> {
    Number(&'s str),
    String(&'s str),
}
