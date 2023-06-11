use crate::{ast::Expr, token::TokenKind};

use super::{Error, TokenStream};

pub fn parse<S: TokenStream>(stream: &mut S) -> Result<Expr, Error> {
    let token = stream.next();
    match token.kind {
        TokenKind::Integer(v) => Ok(Expr::Integer(v)),
        TokenKind::Unknown(c) => Err(Error {
            msg: format!("Unknown character '{}'.", c as char).into_boxed_str(),
            pos: token.pos,
        }),
        TokenKind::End => Err(Error {
            msg: format!("Expected primary value but reached end of code.").into_boxed_str(),
            pos: token.pos,
        }),
        _ => Err(Error {
            msg: format!("Unexpected token.").into_boxed_str(),
            pos: token.pos,
        }),
    }
}
