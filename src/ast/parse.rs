use crate::lexer::*;

use super::{Expr, Operator};

fn expr_digit() -> impl Fn(Input) -> Res<Expr> {
    map(digit(), |d| Expr::Integer(d as i64))
}

fn operator() -> impl Fn(Input) -> Res<Operator> {
    map(or(sym('+'), sym('-')), |c| match c {
        '+' => Operator::Addict,
        '-' => Operator::Subtract,
        _ => panic!("Uncnown operator '{c}'"),
    })
}

pub fn binary() -> impl Fn(Input) -> Res<Expr> {
    let other = and(operator(), expr_digit());
    let others = fold(
        other,
        || Vec::new(),
        |mut v, e| {
            v.push(e);
            v
        },
    );
    map(and(expr_digit(), others), |(first, others)| {
        if others.is_empty() {
            first
        } else {
            Expr::Binary {
                first: Box::new(first),
                others,
            }
        }
    })
}
