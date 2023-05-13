/// Source code byte position
pub type Pos = u32;

/// Syntax context tier
pub type Tier = u32;

#[derive(Debug)]
#[derive_const(Default)]
pub struct Span {
    // start byte pos
    pub lo: Pos,
    // end byte pos
    pub hi: Pos,
    // syntax context tier
    pub tier: Tier,
}

impl Span {
    pub const DUMMY: Self = Self::default();
}
