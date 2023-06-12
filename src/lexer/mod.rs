use crate::token::{Token, TokenKind};

mod reader;

pub use reader::*;

fn is_whitespace(c: u8) -> bool {
    c == b' ' || c == b'\t' || c == b'\r' || c == b'\n'
}

fn skip_whitespaces<R: Reader>(reader: &mut R) {
    while let Some(c) = reader.current() {
        if is_whitespace(c) {
            reader.next();
        } else {
            break;
        }
    }
}

pub fn lex<R: Reader>(reader: &mut R) -> Token {
    skip_whitespaces(reader);

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
