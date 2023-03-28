/// [Comment][1]
///
/// [1]:https://tc39.es/ecma262/#sec-comments
#[derive(Debug)]
pub enum Comment<'s> {
    /// Single line
    /// - //
    Line(&'s str),
    /// Multi line
    /// - /* */
    Block(&'s str),
}
