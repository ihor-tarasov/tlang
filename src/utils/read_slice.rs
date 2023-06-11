use std::io::Read;

pub struct ReadSlice<'a>(&'a [u8]);

impl<'a> ReadSlice<'a> {
    pub fn new(slice: &'a [u8]) -> Self {
        Self(slice)
    }
}

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
