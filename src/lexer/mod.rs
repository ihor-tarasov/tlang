use crate::token::{Token, TokenKind};

mod reader;

pub use reader::*;

pub fn lex<R: Reader>(reader: &mut R) -> Token {
    let start = reader.pos();

    let c = match reader.next() {
        Some(c) => c,
        None => {
            return Token {
                kind: TokenKind::End,
                pos: start..(start + 1),
            }
        }
    };

    let kind = match c {
        b'0'..=b'9' => TokenKind::Integer((c - b'0') as i64),
        b'+' | b'-' => TokenKind::Symbol(c),
        _ => TokenKind::Unknown(c),
    };

    let end = reader.pos();

    Token {
        kind,
        pos: start..end,
    }
}
