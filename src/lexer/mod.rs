use std::{iter::Enumerate, ops::Range, str::Chars};

#[derive(Debug)]
pub struct Pos(Range<usize>);

impl Pos {
    pub fn new_one(p: usize) -> Self {
        Self(p..(p + 1))
    }

    pub fn merge(l: Self, r: Self) -> Self {
        Self(l.0.start..r.0.end)
    }
}

#[derive(Debug)]
pub struct Msg(Box<str>);

#[derive(Debug)]
pub struct Err {
    msg: Msg,
    pos: Pos,
}

pub struct Lexem<'a, T> {
    t: T,
    i: Input<'a>,
    p: Pos,
}

impl<'a, T> Lexem<'a, T> {
    pub fn new(t: T, i: Input<'a>, p: Pos) -> Self {
        Self { t, i, p }
    }

    pub fn data(self) -> T {
        self.t
    }
}

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

pub fn any(i: Input) -> Res<char> {
    i.next()
}

pub fn filter<T, P, F>(p: P, f: F) -> impl Fn(Input) -> Res<T>
where
    P: Fn(Input) -> Res<T>,
    F: Fn(&T) -> bool,
{
    move |i| {
        p(i).and(|l| {
            if f(&l.t) {
                Res::new_lexem(l)
            } else {
                Res::new_empty()
            }
        })
    }
}

pub fn map<T, E, P, F>(p: P, f: F) -> impl Fn(Input) -> Res<E>
where
    P: Fn(Input) -> Res<T>,
    F: Fn(T) -> E,
{
    move |i| p(i).and(|l| Res::new_lexem(Lexem::new(f(l.t), l.i, l.p)))
}

pub fn fold<T, E, P, C, F>(p: P, c: C, f: F) -> impl Fn(Input) -> Res<E>
where
    P: Fn(Input) -> Res<T>,
    C: Fn() -> E,
    F: Fn(E, T) -> E,
{
    move |mut i| {
        let mut e = c();
        let mut start = None;
        let mut end = None;
        loop {
            match p(i.clone()).0 {
                Ok(o) => match o {
                    Some(l) => {
                        i = l.i;
                        if start.is_none() {
                            start = Some(l.p.0.start);
                        }
                        end = Some(l.p.0.end);
                        e = f(e, l.t);
                    }
                    None => break,
                },
                Result::Err(e) => return Res::new_err(e),
            }
        }
        let pos = Pos(start.unwrap_or(0)..end.unwrap_or(0));
        Res::new_lexem(Lexem::new(e, i, pos))
    }
}

pub fn and<T, E, L, R>(l: L, r: R) -> impl Fn(Input) -> Res<(T, E)>
where
    L: Fn(Input) -> Res<T>,
    R: Fn(Input) -> Res<E>,
{
    move |i| {
        l(i).and(|left| {
            r(left.i).and(|right| {
                Res::new_lexem(Lexem::new(
                    (left.t, right.t),
                    right.i,
                    Pos::merge(left.p, right.p),
                ))
            })
        })
    }
}

pub fn or<T, L, R>(l: L, r: R) -> impl Fn(Input) -> Res<T>
where
    L: Fn(Input) -> Res<T>,
    R: Fn(Input) -> Res<T>,
{
    move |i| l(i.clone()).or(|| r(i))
}

pub fn digit() -> impl Fn(Input) -> Res<i64> {
    map(filter(any, |c| c.is_ascii_digit()), |c| {
        c.to_digit(10).unwrap() as i64
    })
}

pub fn sym(c: char) -> impl Fn(Input) -> Res<char> {
    filter(any, move |fc| *fc == c)
}


