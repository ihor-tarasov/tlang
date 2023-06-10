use std::io::Write;

use crate::ast::{self, Operator};

use super::{inst::*, type_io::WriteType, Error, Res};

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
