pub trait Reader {
    fn current(&mut self) -> Option<u8>;
    fn next(&mut self) -> Option<u8>;
    fn pos(&self) -> usize;
}
