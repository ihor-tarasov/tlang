use std::{fmt, ops::Range};

use crate::lexer::Reader;

use super::IterToReader;

pub struct LineInfo {
    pub line_index: usize,
    pub char_index: usize,
    pub length: usize,
    pub line: String,
}

fn calculate_line_index_and_offset<R: Reader>(mut reader: R, pos: usize) -> (usize, usize) {
    let mut line_index = 0;
    let mut line_offset = 0;
    let mut offset = reader.pos();
    while let Some(c) = reader.next() {
        if offset == pos {
            break;
        }
        if c == b'\n' {
            line_index += 1;
            line_offset = offset + 1;
        }
        offset = reader.pos();
    }
    (line_index, line_offset)
}

fn read_line<R: Reader>(mut reader: R, line_offset: usize) -> String {
    let mut offset = reader.pos();
    loop {
        if offset == line_offset {
            break;
        }
        match reader.next() {
            Some(_c) => offset = reader.pos(),
            None => break,
        }
    }

    let mut line = String::new();
    while let Some(c) = reader.next() {
        if c == b'\r' || c == b'\n' {
            break;
        }
        line.push(c as char);
    }
    line
}

impl LineInfo {
    pub fn from_reader<R: Reader>(reader: R, pos: Range<usize>) -> Self {
        let (line_index, line_offset) = calculate_line_index_and_offset(reader.clone(), pos.start);
        let line = read_line(reader, line_offset);

        Self {
            line_index,
            char_index: pos.start - line_offset,
            length: pos.end - pos.start,
            line,
        }
    }

    pub fn from_str(code: &'static str, pos: Range<usize>) -> Self {
        Self::from_reader(IterToReader::new(code.as_bytes().iter().cloned()), pos)
    }
}

impl fmt::Display for LineInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "Line: {}, Char: {}",
            self.line_index + 1,
            self.char_index + 1
        )?;
        writeln!(f, "{}", self.line)?;
        for _ in 0..self.char_index {
            write!(f, " ")?;
        }
        for _ in 0..self.length {
            write!(f, "^")?;
        }
        write!(f, "")
    }
}
