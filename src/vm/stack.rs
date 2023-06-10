use super::{Error, Res, Value};

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
