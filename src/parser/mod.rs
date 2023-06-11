mod binary;
mod error;
mod primary;
mod stream;

pub use error::*;
pub use stream::*;

use crate::{ast::Expr, token::TokenKind};

pub fn parse<S: TokenStream>(stream: &mut S) -> Result<Expr, Error> {
    let expr = binary::parse(stream)?;

    let token = stream.next();
    match token.kind {
        TokenKind::End => Ok(Expr::End(Box::new(expr))),
        _ => Err(Error {
            msg: format!("Expected end of code.").into_boxed_str(),
            pos: token.pos,
        }),
    }
}
