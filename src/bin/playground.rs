#[repr(u8)]
#[derive(Clone, Copy, Debug,PartialEq, Eq, IntEnum)]
pub enum TokenColor {
    RED = 0,
    GREEN = 1,
}

#[repr(usize)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntEnum, PartialOrd)]
pub enum Size {
    BIG = 2,
    MID = 1,
    SMALL = 0,
}


pub struct Token {
    color: TokenColor,
    size: Size,
}


pub struct Block {
    tokens: Vec<Token>,
}

pub struct Board {
    pub plate: [[Block; 3]; 3],
}


fn main() {
    let a = "asd";
    let num:i32 = a.parse().unwrap_or_else();
}