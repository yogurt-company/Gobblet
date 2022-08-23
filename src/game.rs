use std::fmt;

#[derive(Debug,Clone)]
pub enum Color {
    RED,
    GREEN,
}

#[derive(Debug,Clone)]
pub enum Size {
    BIG,
    MID,
    SMALL,
}

#[derive(Debug,Clone)]
pub struct Token {
    color : Color,
    size : Size,
}
impl Token {
    pub fn new(color: Color, size: Size) -> Token {
        Token {
            color: color,
            size: size
        }
    }
}

#[derive(Debug,Clone)]
pub struct Block {
    tokens : Vec<Token>,
}

impl Block {
    pub fn new(tokens: Vec<Token>) -> Block {
        Block {
            tokens
        }
    }
    pub fn get_outermost_token(&self) -> Option<Token> {
        self.tokens.last().cloned()
    }
    pub fn pop_outermost_token(&mut self) -> Option<Token>  {
        let last_token = self.tokens.last().cloned();
        self.tokens.remove(self.tokens.len() - 1);
        last_token
    }


}



#[derive(Debug,Default,Clone)]
pub struct Board {
    tokens : Vec<Token>,
}