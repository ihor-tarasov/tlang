use tlang::{
    ast::Compile,
    parser,
    utils::ReadSlice,
    vm::{Compiler, Fixed, Stack, State},
};

fn main() {
    let code = "2+2-1+8-9+4";
    let mut stream = tlang::stream_from_str(code);
    let expr = parser::parse(&mut stream).unwrap();
    let mut program = Vec::new();
    let mut compiler = Compiler::new(&mut program);
    expr.compile(&mut compiler).unwrap();
    let mut state = State::new(Stack::new(Fixed::<256>::new()));
    let res = tlang::vm::run(ReadSlice::new(&program), &mut state).unwrap();
    println!("{res:?}");
}
