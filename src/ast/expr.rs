use super::{Compile, Compiler, Operator};

#[derive(Debug)]
pub enum Expr {
    Integer(i64),
    Binary {
        first: Box<Expr>,
        others: Vec<(Operator, Expr)>,
    },
}

impl Expr {
    fn compile_inner<E, C: Compiler<E>>(&self, compiler: &mut C) -> Result<(), E> {
        match self {
            Expr::Integer(v) => compiler.integer(*v),
            Expr::Binary { first, others } => {
                first.compile_inner(compiler)?;
                for (operator, expr) in others {
                    compiler.binary()?;
                    expr.compile_inner(compiler)?;
                    compiler.operator(*operator)?;
                }
                Ok(())
            }
        }
    }
}

impl Compile for Expr {
    fn compile<E, C: Compiler<E>>(&self, compiler: &mut C) -> Result<(), E> {
        self.compile_inner(compiler)?;
        compiler.end()
    }
}
