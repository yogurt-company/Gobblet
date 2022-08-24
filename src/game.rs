use std::fmt;
use int_enum::IntEnum;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntEnum)]
pub enum Color {
    RED = 0,
    GREEN = 1,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntEnum, PartialOrd)]
pub enum Size {
    BIG = 2,
    MID = 1,
    SMALL = 0,
}

#[derive(Debug, Clone, Copy)]
pub struct Token {
    color : Color,
    size : Size,
}
impl Token {
    pub fn new(color: Color, size: Size) -> Token {
        Token {
            color,
            size
        }
    }
}

#[derive(Debug,Clone,Default)]
pub struct Block {
    tokens : Vec<Token>,
}

impl Block {
    pub fn new(tokens: Vec<Token>) -> Block {
        Block {
            tokens
        }
    }
    pub fn default() -> Block {
        Block {
            tokens : Vec::new()
        }
    }
    pub fn get_outermost_token(&self) -> Option<Token> {
        self.tokens.last().cloned()
    }
    pub fn pop_outermost_token(&mut self) -> Token {
        self.tokens.remove(self.tokens.len() - 1)
    }
    pub fn is_stackable( & self , token:Token) -> bool {
        match self.get_outermost_token() {
            Some(t) => {
                if t.color != token.color && t.size < token.size {
                    return true;
                }
            }
            None => {
                return true;
            }
        }
        false
    }
    // push a token to the block, return true if successful
    pub fn push_token(&mut self, token:Token) -> bool {
        if self.is_stackable(token) {
            self.tokens.push(token);
            true
        } else {
            false
        }
    }
}



#[derive(Debug,Default,Clone)]
pub struct Board {
    pub plate : [[Block;3]; 3],
}

impl Board {
    pub fn new(blocks: [[Block; 3]; 3]) -> Board {
        Board {
            plate : blocks
        }
    }
}
