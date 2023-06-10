use super::{Lexem, Err};

pub struct Res<'a, T>(Result<Option<Lexem<'a, T>>, Err>);

impl<'a, T> Res<'a, T> {
    pub fn new_lexem(l: Lexem<'a, T>) -> Self {
        Self(Ok(Some(l)))
    }

    pub fn new_empty() -> Self {
        Self(Ok(None))
    }

    pub fn new_err(e: Err) -> Self {
        Self(Result::Err(e))
    }

    pub fn and<E, F>(self, f: F) -> Res<'a, E>
    where
        F: FnOnce(Lexem<'a, T>) -> Res<'a, E>,
    {
        match self.0 {
            Ok(o) => match o {
                Some(l) => f(l),
                None => Res::new_empty(),
            },
            Result::Err(e) => Res::new_err(e),
        }
    }

    pub fn or<F>(self, f: F) -> Res<'a, T>
    where
        F: FnOnce() -> Res<'a, T>,
    {
        match self.0 {
            Ok(o) => match o {
                Some(l) => Res::new_lexem(l),
                None => f(),
            },
            Result::Err(e) => Res::new_err(e),
        }
    }

    pub fn lexem(self) -> Result<Option<Lexem<'a, T>>, Err> {
        self.0
    }
}
