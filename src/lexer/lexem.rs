use super::{Input, Pos};

pub struct Lexem<'a, T> {
    pub t: T,
    pub i: Input<'a>,
    pub p: Pos,
}

impl<'a, T> Lexem<'a, T> {
    pub fn new(t: T, i: Input<'a>, p: Pos) -> Self {
        Self { t, i, p }
    }
}
