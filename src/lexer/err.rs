use super::Pos;

#[derive(Debug)]
pub struct Msg(Box<str>);

#[derive(Debug)]
pub struct Err {
    pub msg: Msg,
    pub pos: Pos,
}
