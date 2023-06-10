use std::io::{Read, Write};

use crate::ast::{Operator, self};

#[derive(Debug)]
pub enum Error {
    IOError,
    StackOverflow,
    StackUnderflow,
    Custom(Box<str>),
    UnknownInstruction(u8),
}

pub type Res<T> = Result<T, Error>;

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
    P: WriteBytes<{ std::mem::size_of::<i64>() }>,
{
    fn write_type(&mut self, t: i64) -> Res<()> {
        self.write_bytes(t.to_be_bytes())
    }
}

impl<P> WriteType<u8> for P
where
    P: WriteBytes<{ std::mem::size_of::<u8>() }>,
{
    fn write_type(&mut self, t: u8) -> Res<()> {
        self.write_bytes(t.to_be_bytes())
    }
}

impl<P> ReadType<i64> for P
where
    P: ReadBytes<{ std::mem::size_of::<i64>() }>,
{
    fn read_type(&mut self) -> Res<i64> {
        Ok(i64::from_be_bytes(self.read_bytes()?))
    }
}

impl<P> ReadType<u8> for P
where
    P: ReadBytes<{ std::mem::size_of::<u8>() }>,
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

pub struct Compiler<'a, W>(&'a mut W);

impl<'a, W> Compiler<'a, W> {
    pub fn new(w: &'a mut W) -> Self {
        Self(w)
    }
}

impl<'a, W: Write> ast::Compiler<Error> for Compiler<'a, W> {
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
pub enum Value {
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

pub struct Fixed<const COUNT: usize>([Value; COUNT]);

impl<const COUNT: usize> Fixed<COUNT> {
    pub fn new() -> Self {
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

pub trait Push {
    fn push(&mut self, v: Value) -> Res<()>;
}

pub trait Pop {
    fn pop(&mut self) -> Res<Value>;
}

pub struct Stack<D> {
    data: D,
    current: usize,
}

impl<D> Stack<D> {
    pub fn new(data: D) -> Self {
        Self { data, current: 0 }
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

pub struct State<S> {
    stack: S,
    accumulator: Value,
}

impl<S: Push + Pop> State<S> {
    pub fn new(stack: S) -> Self {
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
            _ => {
                return Err(Error::Custom(
                    format!(
                        "Unable to use '+' operator for {:?} and {:?} values",
                        l, self.accumulator
                    )
                    .into_boxed_str(),
                ))
            }
        };
        Ok(())
    }

    fn subtract(&mut self) -> Res<()> {
        let l = self.stack.pop()?;
        self.accumulator = match (l, self.accumulator) {
            (Value::Integer(l), Value::Integer(r)) => Value::Integer(l.wrapping_sub(r)),
            _ => {
                return Err(Error::Custom(
                    format!(
                        "Unable to use '-' operator for {:?} and {:?} values",
                        l, self.accumulator
                    )
                    .into_boxed_str(),
                ))
            }
        };
        Ok(())
    }
}

fn step<R: Read, S: Push + Pop>(read: &mut R, state: &mut State<S>) -> Res<bool> {
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

pub fn run<R: Read, S: Push + Pop>(mut read: R, state: &mut State<S>) -> Res<Value> {
    while step(&mut read, state)? {}
    Ok(state.accumulator)
}
