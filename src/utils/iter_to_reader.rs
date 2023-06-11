use crate::lexer::Reader;

pub struct IterToReader<I: Iterator<Item = u8>> {
    iter: I,
    c: Option<u8>,
    pos: usize,
}

impl<I: Iterator<Item = u8>> IterToReader<I> {
    pub fn new(mut iter: I) -> Self {
        Self {
            c: iter.next(),
            iter,
            pos: 0,
        }
    }
}

impl<I: Iterator<Item = u8>> Reader for IterToReader<I> {
    fn current(&mut self) -> Option<u8> {
        self.c.clone()
    }

    fn next(&mut self) -> Option<u8> {
        std::mem::replace(
            &mut self.c,
            self.iter.next().and_then(|c| {
                self.pos += 1;
                Some(c)
            }),
        )
    }

    fn pos(&self) -> usize {
        self.pos
    }
}
