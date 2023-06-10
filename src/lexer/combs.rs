use super::{Input, Res, Lexem, Pos};

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
            match p(i.clone()).lexem() {
                Ok(o) => match o {
                    Some(l) => {
                        i = l.i;
                        if start.is_none() {
                            start = Some(l.p.start());
                        }
                        end = Some(l.p.end());
                        e = f(e, l.t);
                    }
                    None => break,
                },
                Result::Err(e) => return Res::new_err(e),
            }
        }
        let pos = Pos::new(start.unwrap_or(0), end.unwrap_or(0));
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
