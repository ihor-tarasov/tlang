#[derive(Debug)]
pub enum Error {
    IOError,
    StackOverflow,
    StackUnderflow,
    Binary,
    UnknownInstruction,
}

pub type Res<T> = Result<T, Error>;
