use super::{Compile, Compiler, Operator};

#[derive(Debug)]
pub enum Expr {
    Integer(i64),
    Binary {
        first: Box<Expr>,
        others: Vec<(Operator, Expr)>,
    },
    End(Box<Expr>),
}

impl Compile for Expr {
    fn compile<E, C: Compiler<E>>(&self, compiler: &mut C) -> Result<(), E> {
        match self {
            Expr::Integer(v) => compiler.integer(*v),
            Expr::Binary { first, others } => {
                first.compile(compiler)?;
                for (operator, expr) in others {
                    compiler.binary()?;
                    expr.compile(compiler)?;
                    compiler.operator(*operator)?;
                }
                Ok(())
            }
            Expr::End(expr) => {
                expr.compile(compiler)?;
                compiler.end()
            },
        }
    }
}
