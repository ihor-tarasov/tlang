use super::{Compile, Compiler, Binary};

#[derive(Debug)]
pub enum Expr {
    Integer(i64),
    Binary(Binary),
    End(Box<Expr>),
}

impl Compile for Expr {
    fn compile<E, C: Compiler<E>>(&self, compiler: &mut C) -> Result<(), E> {
        match self {
            Expr::Integer(v) => compiler.integer(*v),
            Expr::Binary(binary) => binary.compile(compiler),
            Expr::End(expr) => {
                expr.compile(compiler)?;
                compiler.end()
            },
        }
    }
}
