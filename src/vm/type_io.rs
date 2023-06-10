use std::io::{Write, Read};

use super::{Res, Error};

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

pub trait WriteType<T> {
    fn write_type(&mut self, t: T) -> Res<()>;
}

pub trait ReadType<T> {
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
