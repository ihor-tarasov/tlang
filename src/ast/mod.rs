use crate::lexer::{and, digit, fold, map, or, sym, Input, Res};

#[derive(Debug, Clone, Copy)]
pub enum Operator {
    Addict,
    Subtract,
}

#[derive(Debug)]
pub enum Expr {
    Integer(i64),
    Binary {
        first: Box<Expr>,
        others: Vec<(Operator, Expr)>,
    },
}

pub trait Compiler<E> {
    fn integer(&mut self, v: i64) -> Result<(), E>;
    fn binary(&mut self) -> Result<(), E>;
    fn operator(&mut self, o: Operator) -> Result<(), E>;
    fn end(&mut self) -> Result<(), E>;
}

pub trait Compile {
    fn compile<E, C: Compiler<E>>(&self, compiler: &mut C) -> Result<(), E>;
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

fn expr_digit() -> impl Fn(Input) -> Res<Expr> {
    map(digit(), |d| Expr::Integer(d as i64))
}

fn operator() -> impl Fn(Input) -> Res<Operator> {
    map(or(sym('+'), sym('-')), |c| match c {
        '+' => Operator::Addict,
        '-' => Operator::Subtract,
        _ => panic!("Uncnown operator '{c}'"),
    })
}

pub fn binary() -> impl Fn(Input) -> Res<Expr> {
    let other = and(operator(), expr_digit());
    let others = fold(
        other,
        || Vec::new(),
        |mut v, e| {
            v.push(e);
            v
        },
    );
    map(and(expr_digit(), others), |(first, others)| {
        if others.is_empty() {
            first
        } else {
            Expr::Binary {
                first: Box::new(first),
                others,
            }
        }
    })
}
