use crate::{ast::{Expr, Operator}, token::TokenKind};

use super::{TokenStream, Error, primary};

fn term<S: TokenStream>(stream: &mut S) -> Result<Expr, Error> {
    let first = primary::parse(stream)?;
    let mut others = Vec::new();
    loop {
        let token = stream.current();
        match token.kind {
            TokenKind::Symbol(b'+') => {
                let _token = stream.next();
                let other = primary::parse(stream)?;
                others.push((Operator::Addict, other))
            }
            TokenKind::Symbol(b'-') => {
                let _token = stream.next();
                let other = primary::parse(stream)?;
                others.push((Operator::Subtract, other))
            }
            _ => break,
        }
    }
    if others.is_empty() {
        Ok(first)
    } else {
        Ok(Expr::Binary {
            first: Box::new(first),
            others,
        })
    }
}

pub fn parse<S: TokenStream>(stream: &mut S) -> Result<Expr, Error> {
    term(stream)
}
