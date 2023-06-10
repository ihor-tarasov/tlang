use super::Operator;

pub trait Compiler<E> {
    fn integer(&mut self, v: i64) -> Result<(), E>;
    fn binary(&mut self) -> Result<(), E>;
    fn operator(&mut self, o: Operator) -> Result<(), E>;
    fn end(&mut self) -> Result<(), E>;
}

pub trait Compile {
    fn compile<E, C: Compiler<E>>(&self, compiler: &mut C) -> Result<(), E>;
}
