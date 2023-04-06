pub trait Tokenize<I>
where
    I: Iterator,
{
    fn tokenize(self) -> I;
}
