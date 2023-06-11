use std::ops::Range;

#[derive(Debug)]
pub struct Error {
    pub msg: Box<str>,
    pub pos: Range<usize>,
}
