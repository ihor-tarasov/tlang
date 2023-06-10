use std::{iter::Enumerate, str::Chars};

use super::{Lexem, Pos, Res};

#[derive(Clone)]
pub struct Input<'a>(Enumerate<Chars<'a>>);

impl<'a> Input<'a> {
    pub fn new(c: Chars<'a>) -> Self {
        Self(c.enumerate())
    }

    pub fn next(mut self) -> Res<'a, char> {
        match self.0.next() {
            Some((p, c)) => Res::new_lexem(Lexem::new(c, self, Pos::new_one(p))),
            None => Res::new_empty(),
        }
    }
}
