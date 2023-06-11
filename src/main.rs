use std::io::Read;

use tlang::{
    ast::Compile,
    parser::{self, TokenStream},
    vm::{Compiler, Fixed, Stack, State}, lexer::{Reader, self}, token::Token,
};

struct ReadSlice<'a>(&'a [u8]);

impl<'a> Read for ReadSlice<'a> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let count = self.0.len().min(buf.len());
        for i in 0..count {
            buf[i] = self.0[i];
        }
        self.0 = &self.0[count..];
        Ok(count)
    }
}

pub struct IterToReader<I: Iterator<Item = u8>> {
    iter: I,
    c: Option<u8>,
    pos: usize,
}

impl<I: Iterator<Item = u8>> IterToReader<I> {
    fn new(mut iter: I) -> Self {
        Self {
            c: iter.next(),
            iter,
            pos: 0,
        }
    }
}

impl<I: Iterator<Item = u8>> Reader for IterToReader<I> {
    fn current(&mut self) -> Option<u8> {
        self.c.clone()
    }

    fn next(&mut self) -> Option<u8> {
        std::mem::replace(
            &mut self.c,
            self.iter.next().and_then(|c| {
                self.pos += 1;
                Some(c)
            }),
        )
    }

    fn pos(&self) -> usize {
        self.pos
    }
}

struct Tokenizer<R> {
    reader: R,
    token: Token,
}

impl<R: Reader> Tokenizer<R> {
    fn new(mut reader: R) -> Self {
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

fn main() {
    let code = "2+2-1+8-9+4".as_bytes().iter().cloned();

    let mut stream = Tokenizer::new(IterToReader::new(code));

    let expr = parser::parse(&mut stream).unwrap();

    let mut program = Vec::new();
    let mut compiler = Compiler::new(&mut program);
    expr.compile(&mut compiler).unwrap();

    let mut state = State::new(Stack::new(Fixed::<256>::new()));

    let res = tlang::vm::run(ReadSlice(&program), &mut state).unwrap();

    println!("{res:?}");
}
