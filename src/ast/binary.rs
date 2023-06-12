use super::{Compile, Compiler, Expr, Operator};

#[derive(Debug)]
pub struct Binary {
    first: Box<Expr>,
    others: Vec<(Operator, Expr)>,
}

impl Binary {
    pub fn new(first: Expr, others: Vec<(Operator, Expr)>) -> Expr {
        Expr::Binary(Self {
            first: Box::new(first),
            others,
        })
    }
}

impl Compile for Binary {
    fn compile<E, C: Compiler<E>>(&self, compiler: &mut C) -> Result<(), E> {
        self.first.compile(compiler)?;
        for (operator, expr) in &self.others {
            compiler.binary()?;
            expr.compile(compiler)?;
            compiler.operator(*operator)?;
        }
        Ok(())
    }
}
