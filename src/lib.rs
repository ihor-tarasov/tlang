use parser::TokenStream;
use utils::{Tokenizer, IterToReader};

pub mod ast;
pub mod lexer;
pub mod parser;
pub mod vm;
pub mod token;
pub mod utils;

pub fn stream_from_iter<I: Iterator<Item = u8>>(iter: I) -> impl TokenStream {
    Tokenizer::new(IterToReader::new(iter))
}

pub fn stream_from_str(s: &'static str) -> impl TokenStream {
    stream_from_iter(s.as_bytes().iter().cloned())
}
