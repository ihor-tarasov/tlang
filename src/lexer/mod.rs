mod pos;
mod err;
mod lexem;
mod res;
mod input;
mod combs;

pub use pos::*;
pub use err::*;
pub use lexem::*;
pub use res::*;
pub use input::*;
pub use combs::*;

pub fn any(i: Input) -> Res<char> {
    i.next()
}

pub fn digit() -> impl Fn(Input) -> Res<i64> {
    map(filter(any, |c| c.is_ascii_digit()), |c| {
        c.to_digit(10).unwrap() as i64
    })
}

pub fn sym(c: char) -> impl Fn(Input) -> Res<char> {
    filter(any, move |fc| *fc == c)
}
