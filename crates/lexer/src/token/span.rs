/// Source code byte position
pub type Pos = u32;

/// Syntax context tier
pub type Tier = u32;

#[derive(Debug)]
pub struct Span {
    // start byte pos
    pub lo: Pos,
    // end byte pos
    pub hi: Pos,
    // syntax context tier
    pub tier: Tier,
}

impl const Default for Span {
    fn default() -> Self {
        Self {
            lo: 0u32,
            hi: 0u32,
            tier: 0u32,
        }
    }
}

impl Span {
    pub const DUMMY: Self = Self::default();
}
