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
    plate : Vec<Vec<Block>>,
}

impl Board {
    pub fn new() -> Board {
        Board {
            plate : Vec::new(),
        }
    }
    pub fn get_plate(&self) -> &Vec<Vec<Block>> {
        &self.plate
    }
    pub fn get_plate_mut(&mut self) -> &mut Vec<Vec<Block>> {
        &mut self.plate
    }
    pub fn get_row(&self, row:usize) -> &Vec<Block> {
        &self.plate[row]
    }
    pub fn get_row_mut(&mut self, row:usize) -> &mut Vec<Block> {
        &mut self.plate[row]
    }
    pub fn get_col(&self, col:usize) -> &Vec<Block> {
        let mut col = Vec::new();
        for row in 0..self.plate.len() {
            col.push(self.plate[row][col]);
        }
        &col
    }
    pub fn get_col_mut(&mut self, col:usize) -> &mut Vec<Block> {
        let mut col = Vec::new();
        for row in 0..self.plate.len() {
            col.push(self.plate[row][col]);
        }
        &mut col
    }
    pub fn get_block(&self, row:usize, col:usize) -> &Block {
        &self.plate[row][col]
    }
    pub fn get_block_mut(&mut self, row:usize, col:usize) -> &mut Block {
        &mut self.plate[row][col]
    }
    pub fn get_block_mut_by_token(&mut self, token:Token) -> &mut Block {
        for row in 0..self.plate.len() {
            for col in 0..self.plate[row].len() {
                if self.plate[row][col].get_outermost_token() == Some(token) {
                    return &mut self.plate[row][col];
                }
            }
        }
        panic!("No block found with token {:?}", token);
}
