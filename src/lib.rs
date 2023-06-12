use ast::{Compile, Expr};
use parser::TokenStream;
use utils::{IterToReader, ReadSlice, Tokenizer};
use vm::{Compiler, Error, Fixed, Pop, Push, Res, Stack, State, Value};

use crate::utils::LineInfo;

pub mod ast;
pub mod lexer;
pub mod parser;
pub mod token;
pub mod utils;
pub mod vm;

pub fn stream_from_iter<I: Iterator<Item = u8> + Clone>(iter: I) -> impl TokenStream {
    Tokenizer::new(IterToReader::new(iter))
}

pub fn stream_from_str(s: &'static str) -> impl TokenStream {
    stream_from_iter(s.as_bytes().iter().cloned())
}

pub fn compile_to_vec(expr: &Expr) -> Result<Vec<u8>, Error> {
    let mut program = Vec::new();
    let mut compiler = Compiler::new(&mut program);
    expr.compile(&mut compiler).and_then(|_| Ok(program))
}

pub fn state_fixed<const STACK_SIZE: usize>() -> State<Stack<Fixed<STACK_SIZE>>> {
    State::new(Stack::new(Fixed::<STACK_SIZE>::new()))
}

pub fn run_slice<S: Push + Pop>(slice: &[u8], state: &mut State<S>) -> Res<Value> {
    vm::run(ReadSlice::new(slice), state)
}

#[derive(Debug)]
pub enum RunError {
    VmError(vm::Error),
    ParserError(parser::Error),
}

pub fn run_str(code: &'static str) -> Result<Value, RunError> {
    let expr = parser::parse(stream_from_str(code)).map_err(|e| RunError::ParserError(e))?;
    run_slice(
        &compile_to_vec(&expr).map_err(|e| RunError::VmError(e))?,
        &mut state_fixed::<256>(),
    )
    .map_err(|e| RunError::VmError(e))
}

pub fn run_str_unwrap(code: &'static str) -> Option<Value> {
    match run_str(code) {
        Ok(v) => return Some(v),
        Err(e) => match e {
            RunError::VmError(e) => println!("Runtime Error: {e:?}"),
            RunError::ParserError(e) => {
                let info = LineInfo::from_str(code, e.pos);
                println!("{info}");
                println!("Parse error: {}", e.msg);
            }
        },
    }
    None
}
