use super::{Error, Pop, Push, Res, Value};

pub struct State<S> {
    stack: S,
    pub accumulator: Value,
    error: Option<Box<str>>,
}

impl<S: Push + Pop> State<S> {
    pub fn new(stack: S) -> Self {
        Self {
            stack,
            accumulator: Value::Void,
            error: None,
        }
    }

    pub fn load(&mut self, v: Value) -> Res<()> {
        self.accumulator = v;
        Ok(())
    }

    pub fn integer(&mut self, i: i64) -> Res<()> {
        self.load(Value::Integer(i))
    }

    pub fn push(&mut self) -> Res<()> {
        self.stack.push(self.accumulator)
    }

    pub fn pop(&mut self) -> Res<()> {
        self.accumulator = self.stack.pop()?;
        Ok(())
    }

    pub fn error<T>(&mut self, msg: String, e: Error) -> Res<T> {
        self.error = Some(msg.into_boxed_str());
        Err(e)
    }

    pub fn addict(&mut self) -> Res<()> {
        let l = self.stack.pop()?;
        self.accumulator = match (l, self.accumulator) {
            (Value::Integer(l), Value::Integer(r)) => Value::Integer(l.wrapping_add(r)),
            _ => {
                return self.error(
                    format!(
                        "Unable to use '+' operator for {:?} and {:?} values",
                        l, self.accumulator
                    ),
                    Error::Binary,
                )
            }
        };
        Ok(())
    }

    pub fn subtract(&mut self) -> Res<()> {
        let l = self.stack.pop()?;
        self.accumulator = match (l, self.accumulator) {
            (Value::Integer(l), Value::Integer(r)) => Value::Integer(l.wrapping_sub(r)),
            _ => {
                return self.error(
                    format!(
                        "Unable to use '-' operator for {:?} and {:?} values",
                        l, self.accumulator
                    ),
                    Error::Binary,
                )
            }
        };
        Ok(())
    }
}
