use std::{str::Chars, io::{Write, Read}};

fn any(mut i: Chars) -> Option<(char, Chars)> {
    i.next().and_then(|c| Some((c, i)))
}

fn filter<T, P, F>(p: P, f: F) -> impl Fn(Chars) -> Option<(T, Chars)>
where
    P: Fn(Chars) -> Option<(T, Chars)>,
    F: Fn(&T) -> bool,
{
    move |i| p(i).and_then(|(t, i)| if f(&t) { Some((t, i)) } else { None })
}

fn map<T, E, P, F>(p: P, f: F) -> impl Fn(Chars) -> Option<(E, Chars)>
where
    P: Fn(Chars) -> Option<(T, Chars)>,
    F: Fn(T) -> E,
{
    move |i| p(i).and_then(|(t, i)| Some((f(t), i)))
}

fn filter_map<T, E, P, F>(p: P, f: F) -> impl Fn(Chars) -> Option<(E, Chars)>
where
    P: Fn(Chars) -> Option<(T, Chars)>,
    F: Fn(T) -> Option<E>,
{
    move |i| p(i).and_then(|(t, i)| Some((f(t)?, i)))
}

fn fold<T, E, P, C, F>(p: P, c: C, f: F) -> impl Fn(Chars) -> Option<(E, Chars)>
where
    P: Fn(Chars) -> Option<(T, Chars)>,
    C: Fn() -> E,
    F: Fn(E, T) -> E,
{
    move |mut i| {
        let mut e = c();
        while let Some((t, ni)) = p(i.clone()) {
            i = ni;
            e = f(e, t);
        }
        Some((e, i))
    }
}

fn and<T, E, L, R>(l: L, r: R) -> impl Fn(Chars) -> Option<((T, E), Chars)>
where
    L: Fn(Chars) -> Option<(T, Chars)>,
    R: Fn(Chars) -> Option<(E, Chars)>,
{
    move |i| l(i).and_then(|(t, i)| r(i).and_then(|(e, i)| Some(((t, e), i))))
}

fn or<T, L, R>(l: L, r: R) -> impl Fn(Chars) -> Option<(T, Chars)>
where
    L: Fn(Chars) -> Option<(T, Chars)>,
    R: Fn(Chars) -> Option<(T, Chars)>,
{
    move |i| match l(i.clone()) {
        Some(d) => Some(d),
        None => r(i),
    }
}

fn digit() -> impl Fn(Chars) -> Option<(u32, Chars)> {
    filter_map(any, |c| c.to_digit(10))
}

fn sym(c: char) -> impl Fn(Chars) -> Option<(char, Chars)> {
    filter(any, move |fc| *fc == c)
}

#[derive(Debug, Clone, Copy)]
enum Operator {
    Addict,
    Subtract,
}

#[derive(Debug)]
enum Expr {
    Integer(i64),
    Binary {
        first: Box<Expr>,
        others: Vec<(Operator, Expr)>,
    }
}

impl Expr {
    fn compile_inner<C: Compiler>(&self, compiler: &mut C) -> Res<()> {
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
            },
        }
    }
}

trait Compiler {
    fn integer(&mut self, v: i64) -> Res<()> ;
    fn binary(&mut self) -> Res<()>;
    fn operator(&mut self, o: Operator) -> Res<()>;
    fn end(&mut self) -> Res<()>;
}

trait Compile {
    fn compile<C: Compiler>(&self, compiler: &mut C) -> Res<()>;
}

impl Compile for Expr {
    fn compile<C: Compiler>(&self, compiler: &mut C) -> Res<()> {
        self.compile_inner(compiler)?;
        compiler.end()
    }
}

fn expr_digit() -> impl Fn(Chars) -> Option<(Expr, Chars)> {
    map(digit(), |d| Expr::Integer(d as i64))
}

fn operator() -> impl Fn(Chars) -> Option<(Operator, Chars)> {
    map(or(sym('+'), sym('-')), |c| match c {
        '+' => Operator::Addict,
        '-' => Operator::Subtract,
        _ => panic!("Uncnown operator '{c}'"),
    })
}

fn binary() -> impl Fn(Chars) -> Option<(Expr, Chars)> {
    let other = and(operator(), expr_digit());
    let others = fold(other, || Vec::new(), |mut v, e| {
        v.push(e);
        v
    });
    map(and(expr_digit(), others), |(first, others)| if others.is_empty() {
        first
    } else {
        Expr::Binary { first: Box::new(first), others }
    })
}

#[derive(Debug)]
enum Error {
    IOError,
    StackOverflow,
    StackUnderflow,
    Custom(Box<str>),
    UnknownInstruction(u8),
}

type Res<T> = Result<T, Error>;

trait WriteBytes<const COUNT: usize> {
    fn write_bytes(&mut self, b: [u8; COUNT]) -> Res<()>;
}

trait ReadBytes<const COUNT: usize> {
    fn read_bytes(&mut self) -> Res<[u8; COUNT]>;
}

impl<const COUNT: usize, W: Write> WriteBytes<COUNT> for W {
    fn write_bytes(&mut self, b: [u8; COUNT]) -> Res<()> {
        match self.write_all(&b) {
            Ok(_) => Ok(()),
            Err(_) => Err(Error::IOError),
        }
    }
}

impl<const COUNT: usize, R: Read> ReadBytes<COUNT> for R {
    fn read_bytes(&mut self) -> Res<[u8; COUNT]> {
        let mut buf = [0u8; COUNT];
        match self.read_exact(&mut buf) {
            Ok(_) => Ok(buf),
            Err(_) => Err(Error::IOError),
        }
    }
}

trait WriteType<T> {
    fn write_type(&mut self, t: T) -> Res<()>;
}

trait ReadType<T> {
    fn read_type(&mut self) -> Res<T>;
}

impl<P> WriteType<i64> for P
where
    P: WriteBytes<{std::mem::size_of::<i64>()}>,
{
    fn write_type(&mut self, t: i64) -> Res<()> {
        self.write_bytes(t.to_be_bytes())
    }
}

impl<P> WriteType<u8> for P
where
    P: WriteBytes<{std::mem::size_of::<u8>()}>,
{
    fn write_type(&mut self, t: u8) -> Res<()> {
        self.write_bytes(t.to_be_bytes())
    }
}

impl<P> ReadType<i64> for P
where
    P: ReadBytes<{std::mem::size_of::<i64>()}>,
{
    fn read_type(&mut self) -> Res<i64> {
        Ok(i64::from_be_bytes(self.read_bytes()?))
    }
}

impl<P> ReadType<u8> for P
where
    P: ReadBytes<{std::mem::size_of::<u8>()}>,
{
    fn read_type(&mut self) -> Res<u8> {
        Ok(u8::from_be_bytes(self.read_bytes()?))
    }
}

const END: u8 = 0x00;
const LDI: u8 = 0x01;
const PSH: u8 = 0x02;
const ADD: u8 = 0x03;
const SUB: u8 = 0x04;

struct TVMCompiler<'a, W>(&'a mut W);

impl<'a, W: Write> Compiler for TVMCompiler<'a, W> {
    fn integer(&mut self, v: i64) -> Res<()> {
        self.0.write_type(LDI)?;
        self.0.write_type(v)
    }

    fn binary(&mut self) -> Res<()> {
        self.0.write_type(PSH)
    }

    fn operator(&mut self, o: Operator) -> Res<()> {
        match o {
            Operator::Addict => self.0.write_type(ADD),
            Operator::Subtract => self.0.write_type(SUB),
        }
    }

    fn end(&mut self) -> Res<()> {
        self.0.write_type(END)
    }
}

#[derive(Debug, Clone, Copy)]
enum Value {
    Void,
    Integer(i64),
}

trait Get {
    fn get(&self, index: usize) -> Value;
}

trait Set {
    fn set(&mut self, index: usize, value: Value);
}

trait Len {
    fn len(&self) -> usize;
}

struct Fixed<const COUNT: usize>([Value; COUNT]);

impl<const COUNT: usize> Fixed<COUNT> {
    fn new() -> Self {
        Self([Value::Void; COUNT])
    }
}

impl<const COUNT: usize> Get for Fixed<COUNT> {
    fn get(&self, index: usize) -> Value {
        self.0[index]
    }
}

impl<const COUNT: usize> Set for Fixed<COUNT> {
    fn set(&mut self, index: usize, value: Value) {
        self.0[index] = value;
    }
}

impl<const COUNT: usize> Len for Fixed<COUNT> {
    fn len(&self) -> usize {
        COUNT
    }
}

trait Push {
    fn push(&mut self, v: Value) -> Res<()>;
}

trait Pop {
    fn pop(&mut self) -> Res<Value>;
}

struct Stack<D> {
    data: D,
    current: usize,
}

impl<D> Stack<D> {
    fn new(data: D) -> Self {
        Self {
            data,
            current: 0,
        }
    }
}

impl<D: Set + Len> Push for Stack<D> {
    fn push(&mut self, v: Value) -> Res<()> {
        if self.current < self.data.len() {
            self.current += 1;
            self.data.set(self.current - 1, v);
            Ok(())
        } else {
            Err(Error::StackOverflow)
        }
    }
}

impl<D: Get> Pop for Stack<D> {
    fn pop(&mut self) -> Res<Value> {
        if self.current != 0 {
            self.current -= 1;
            Ok(self.data.get(self.current))
        } else {
            Err(Error::StackUnderflow)
        }
    }
}

struct State<S> {
    stack: S,
    accumulator: Value,
}

impl<S: Push + Pop> State<S> {
    fn new(stack: S) -> Self {
        Self {
            stack,
            accumulator: Value::Void,
        }
    }

    fn load(&mut self, v: Value) -> Res<()> {
        self.accumulator = v;
        Ok(())
    }

    fn integer(&mut self, i: i64) -> Res<()> {
        self.load(Value::Integer(i))
    }

    fn push(&mut self) -> Res<()> {
        self.stack.push(self.accumulator)
    }

    fn pop(&mut self) -> Res<()> {
        self.accumulator = self.stack.pop()?;
        Ok(())
    }

    fn addict(&mut self) -> Res<()> {
        let l = self.stack.pop()?;
        self.accumulator = match (l, self.accumulator) {
            (Value::Integer(l), Value::Integer(r)) => Value::Integer(l.wrapping_add(r)),
            _ => return Err(Error::Custom(format!("Unable to use '+' operator for {:?} and {:?} values", l, self.accumulator).into_boxed_str()))
        };
        Ok(())
    }

    fn subtract(&mut self) -> Res<()> {
        let l = self.stack.pop()?;
        self.accumulator = match (l, self.accumulator) {
            (Value::Integer(l), Value::Integer(r)) => Value::Integer(l.wrapping_sub(r)),
            _ => return Err(Error::Custom(format!("Unable to use '-' operator for {:?} and {:?} values", l, self.accumulator).into_boxed_str()))
        };
        Ok(())
    }
}

fn vm_step<R: Read, S: Push + Pop>(read: &mut R, state: &mut State<S>) -> Res<bool> {
    let instruction: u8 = read.read_type()?;
    match instruction {
        LDI => state.integer(read.read_type()?)?,
        PSH => state.push()?,
        ADD => state.addict()?,
        SUB => state.subtract()?,
        END => return Ok(false),
        _ => return Err(Error::UnknownInstruction(instruction)),
    }
    Ok(true)
}

fn vm_run<R: Read, S: Push + Pop>(mut read: R, state: &mut State<S>) -> Res<Value> {
    while vm_step(&mut read, state)? {}
    Ok(state.accumulator)
}

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

    let expr = parser(code).unwrap().0;

    let mut program = Vec::new();
    let mut compiler = TVMCompiler(&mut program);
    expr.compile(&mut compiler).unwrap();

    let mut state = State::new(Stack::new(Fixed::<256>::new()));

    let res = vm_run(ReadSlice(&program), &mut state).unwrap();

    println!("{res:?}");
}
