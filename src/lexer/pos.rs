use std::ops::Range;

#[derive(Debug)]
pub struct Pos(Range<usize>);

impl Pos {
    pub fn new(s: usize, e: usize) -> Self {
        Self(s..e)
    }

    pub fn new_one(p: usize) -> Self {
        Self(p..(p + 1))
    }

    pub fn merge(l: Self, r: Self) -> Self {
        Self(l.0.start..r.0.end)
    }

    pub fn start(&self) -> usize {
        self.0.start
    }

    pub fn end(&self) -> usize {
        self.0.end
    }
}
