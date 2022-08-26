use int_enum::IntEnum;
use std::fmt;
use rstest::*;
use std::collections::HashMap;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntEnum)]
pub enum Color {
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

#[derive(Debug, Clone, Copy)]
pub struct Token {
    color: Color,
    size: Size,
}
impl Token {
    pub fn new(color: Color, size: Size) -> Token {
        Token { color, size }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Block {
    tokens: Vec<Token>,
}

impl Block {
    pub fn new(tokens: Vec<Token>) -> Block {
        Block { tokens }
    }
    pub fn default() -> Block {
        Block { tokens: Vec::new() }
    }
    pub fn get_outermost_token(&self) -> Option<Token> {
        self.tokens.last().cloned()
    }
    pub fn pop_outermost_token(&mut self) -> Token {
        self.tokens.remove(self.tokens.len() - 1)
    }
    pub fn is_stackable(&self, token: Token) -> bool {
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
    pub fn push_token(&mut self, token: Token) -> bool {
        if self.is_stackable(token) {
            self.tokens.push(token);
            true
        } else {
            false
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Board {
    pub plate: [[Block; 3]; 3],
}

    // def is_valid_take_from_board(self, x: int, y: int):
    //     if 0 <= x < 3 and 0 <= y < 3:
    //         block:Block = self.plate[y][x]
    //         token_being_taken = block.pop_outermost()
    //         self.plate[y][x] = block
    //         if self.is_end():
    //             return token_being_taken
    //         else:
    //             return False
    //     else:
    //         return False

impl Board {
    pub fn new(blocks: [[Block; 3]; 3]) -> Board {
        Board { plate: blocks }
    }

    fn pattern_check_fp(&self, pattern: [[usize; 2]; 3]) -> Option<Color> {
        let result = pattern
            .iter()
            .map(|axis| self.plate[axis[0]][axis[1]].get_outermost_token())
            .filter(|t| t.is_some())
            .map(|t| t.unwrap().color)
            .fold([0, 0], |acc, c| match c {
                Color::RED => [acc[0] + 1, acc[1]],
                Color::GREEN => [acc[0], acc[1] + 1],
                _ => acc,
            });
        match result {
            [3, 0] => Some(Color::RED),
            [0, 3] => Some(Color::GREEN),
            _ => None,
        }
    }
    pub fn is_gameover(&self) -> Option<Color> {
        let patterns:[[[usize;2];3];8] = [
            [[2, 0], [1, 1], [0, 2]],
            [[0, 0], [1, 1], [2, 2]],
            [[0, 0], [0, 1], [0, 2]],
            [[1, 0], [1, 1], [1, 2]],
            [[2, 0], [2, 1], [2, 2]],
            [[0, 0], [1, 0], [2, 0]],
            [[0, 1], [1, 1], [2, 1]],
            [[0, 2], [1, 2], [2, 2]],
        ];
        for pattern in patterns {
            match self.pattern_check_fp(pattern) {
                Some(color) => {
                    return Some(color);
                }
                None => {
                    continue;
                }
            }
        }
        None
    }

    pub fn is_valid_take_from_board(&self, x: usize, y: usize) -> bool {
        if x < 3 && y < 3 {
            if self.plate[y][x].get_outermost_token().is_some() {
                return true;
            }
        }
        false
    }
}


struct Player {
    color: Color,
    inventory: [i8;3],
}
impl Player {
    pub fn new(color:Color) -> Player {
        Player {
            color,
            inventory: [2,2,2],
        }
    }

    pub fn get_token(&mut self, size:Size) -> Option<Token> {
        if self.inventory[size as usize] > 0 {
            self.inventory[size as usize] -= 1;
            Some(Token::new(self.color, size))
        } else {
            None
        }
    }
    
    pub fn place_from_inventory(&mut self, size:Size, board:&mut Board, x:usize, y:usize) -> bool {
        if self.inventory[size as usize] > 0 {
            if board.plate[y][x].push_token(Token::new(self.color, size)) {
                self.inventory[size as usize] -= 1;
                return true;
            }
        }
        false
    }

    pub fn place_from_board(&mut self, board:&mut Board, x:usize, y:usize, x2:usize, y2:usize) -> bool {
        if board.is_valid_take_from_board(x, y) {
            let token = board.plate[y][x].pop_outermost_token();
            if board.plate[y2][x2].push_token(token) {
                return true;
            }
            return false
        }
        return false
    }

}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[fixture]
    fn end_board() -> Board {
        Board::new([
            [
                Block::new(vec![Token::new(Color::RED, Size::BIG)]),
                Block::new(vec![Token::new(Color::GREEN, Size::BIG)]),
                Block::new(vec![Token::new(Color::GREEN, Size::BIG)]),
            ],
            [
                Block::new(vec![Token::new(Color::GREEN, Size::BIG)]),
                Block::new(vec![Token::new(Color::RED, Size::BIG)]),
                Block::new(vec![Token::new(Color::GREEN, Size::BIG)]),
            ],
            [
                Block::new(vec![Token::new(Color::GREEN, Size::BIG)]),
                Block::new(vec![Token::new(Color::GREEN, Size::BIG)]),
                Block::new(vec![Token::new(Color::RED, Size::BIG)]),
            ],
        ])
    }

    #[rstest]
    fn test_end_game(end_board: Board) {
        assert!(end_board.is_gameover() == Some(Color::RED));
    }
    #[test]
    fn test_not_end_game() {
        let end_game_board = Board::new([
            [
                Block::new(vec![Token::new(Color::RED, Size::BIG)]),
                Block::new(vec![Token::new(Color::GREEN, Size::BIG)]),
                Block::new(vec![Token::new(Color::GREEN, Size::BIG)]),
            ],
            [
                Block::new(vec![Token::new(Color::GREEN, Size::BIG)]),
                Block::new(vec![]),
                Block::new(vec![Token::new(Color::GREEN, Size::BIG)]),
            ],
            [
                Block::new(vec![Token::new(Color::GREEN, Size::BIG)]),
                Block::new(vec![Token::new(Color::GREEN, Size::BIG)]),
                Block::new(vec![Token::new(Color::RED, Size::BIG)]),
            ],
        ]);
        assert!(end_game_board.is_gameover().is_none());
    }

}


