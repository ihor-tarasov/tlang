use crate::token::Token;

pub trait TokenStream {
    fn current(&self) -> &Token;
    fn next(&mut self) -> Token;
}
