use std::io::Read;

use tlang::{ast::{binary, Compile}, lexer::Input, vm::{Compiler, State, Stack, Fixed}};

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

fn main() {
    let code = "2+2-1+8-9+1".chars();

    let parser = binary();

    let expr = parser(Input::new(code)).lexem().unwrap().unwrap().t;

    let mut program = Vec::new();
    let mut compiler = Compiler::new(&mut program);
    expr.compile(&mut compiler).unwrap();

    let mut state = State::new(Stack::new(Fixed::<256>::new()));

    let res = tlang::vm::run(ReadSlice(&program), &mut state).unwrap();

    println!("{res:?}");
}
