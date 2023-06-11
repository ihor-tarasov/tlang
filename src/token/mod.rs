use std::ops::Range;

pub enum TokenKind {
    Integer(i64),
    Symbol(u8),
    Unknown(u8),
    End,
}

pub struct Token {
    pub kind: TokenKind,
    pub pos: Range<usize>,
}
