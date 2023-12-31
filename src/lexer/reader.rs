pub trait Reader: Clone {
    fn current(&mut self) -> Option<u8>;
    fn next(&mut self) -> Option<u8>;
    fn pos(&self) -> usize;
}
