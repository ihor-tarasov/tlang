use std::io::Read;

mod compiler;
mod res;
mod stack;
mod state;
mod type_io;
mod value;

pub mod inst;

pub use compiler::*;
pub use res::*;
pub use stack::*;
pub use state::*;
pub use value::*;

use inst::*;
use type_io::*;

fn step<R: Read, S: Push + Pop>(read: &mut R, state: &mut State<S>) -> Res<bool> {
    let instruction: u8 = read.read_type()?;
    match instruction {
        LDI => state.integer(read.read_type()?)?,
        PSH => state.push()?,
        ADD => state.addict()?,
        SUB => state.subtract()?,
        END => return Ok(false),
        _ => {
            return state.error(
                format!("Unknown instruction 0x{:02X}", instruction),
                Error::UnknownInstruction,
            )
        }
    }
    Ok(true)
}

pub fn run<R: Read, S: Push + Pop>(mut read: R, state: &mut State<S>) -> Res<Value> {
    while step(&mut read, state)? {}
    Ok(state.accumulator)
}
