use crate::{
    lexer::{self, Reader},
    parser::TokenStream,
    token::Token,
};

pub struct Tokenizer<R> {
    reader: R,
    token: Token,
}

impl<R: Reader> Tokenizer<R> {
    pub fn new(mut reader: R) -> Self {
        Self {
            token: lexer::lex(&mut reader),
            reader,
        }
    }
}

impl<R: Reader> TokenStream for Tokenizer<R> {
    fn current(&self) -> &Token {
        &self.token
    }

    fn next(&mut self) -> Token {
        std::mem::replace(&mut self.token, lexer::lex(&mut self.reader))
    }
}
