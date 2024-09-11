use std::io;
use std::ops::Not;

use colored::*;
use int_enum::IntEnum;
use rand::Rng;
use rstest::*;
use uuid::Uuid;
use synthesis::game::*;

const GAME_SIZE: usize = 3;


#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum PlayerId {
    RED,
    GREEN,
}
impl Not for PlayerId {
    type Output = Self;
    fn not(self) -> Self::Output {
        match self {
            PlayerId::RED => PlayerId::GREEN,
            PlayerId::GREEN => PlayerId::RED,
        }
    }
}

impl HasTurnOrder for PlayerId {
    fn next(&self) -> Self {
        match self {
            PlayerId::RED => PlayerId::GREEN,
            PlayerId::GREEN => PlayerId::RED,
        }
    }
    fn prev(&self) -> Self {
        match self {
            PlayerId::RED => PlayerId::GREEN,
            PlayerId::GREEN => PlayerId::RED,
        }
    }
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
    color: PlayerId,
    size: Size,
}

impl Token {
    pub fn new(color: PlayerId, size: Size) -> Token {
        Token { color, size }
    }
    pub fn to_string(&self) -> String {
        match self.color {
            PlayerId::RED => match self.size {
                Size::BIG => "🔴".red().bold().to_string(),
                Size::MID => "🔴".red().to_string(),
                Size::SMALL => "🔴".red().dimmed().to_string(),
            },
            PlayerId::GREEN => match self.size {
                Size::BIG => "🟢".green().bold().to_string(),
                Size::MID => "🟢".green().to_string(),
                Size::SMALL => "🟢".green().dimmed().to_string(),
            },
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Block {
    tokens: Vec<Token>,
}

impl Block {
    pub fn new(tokens: Vec<Token>) -> Block {
        Block { tokens }
    }
    pub fn get_outermost_token(&self) -> Option<Token> {
        self.tokens.last().cloned()
    }
    // TODO from copy to borrow, but how?
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

#[derive(Debug, Clone)]
pub struct Board {
    pub plate: [[Block; 3]; 3],
}

impl Board {
    pub fn new(blocks: [[Block; 3]; 3]) -> Board {
        Board { plate: blocks }
    }
    fn pattern_check_fp(&self, pattern: [[usize; 2]; 3]) -> Option<PlayerId> {
        let result = pattern
            .iter()
            .map(|axis| self.plate[axis[0]][axis[1]].get_outermost_token())
            .filter(|t| t.is_some())
            .map(|t| t.unwrap().color)
            .fold([0, 0], |acc, c| match c {
                PlayerId::RED => [acc[0] + 1, acc[1]],
                PlayerId::GREEN => [acc[0], acc[1] + 1],
                _ => acc,
            });
        match result {
            [3, 0] => Some(PlayerId::RED),
            [0, 3] => Some(PlayerId::GREEN),
            _ => None,
        }
    }
    // TODO NxN genernalize
    pub fn is_gameover(&self) -> Option<PlayerId> {
        let patterns: [[[usize; 2]; 3]; 8] = [
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


    // TODO Add Json Status format generator
    pub fn display(&self) {
        for row in self.plate.iter() {
            for block in row.iter() {
                match block.get_outermost_token() {
                    Some(token) => {
                        print!("{} ", token.to_string());
                    }
                    None => {
                        print!(" X ");
                    }
                }
            }
            println!();
        }
    }
}

struct Player {
    color: PlayerId,
    inventory: [u8; 3],
}

impl Player {
    pub fn new(color: PlayerId) -> Player {
        Player {
            color,
            inventory: [2, 2, 2],
        }
    }

    pub fn get_token(&mut self, size: Size) -> Option<Token> {
        if self.inventory[size as usize] > 0 {
            self.inventory[size as usize] -= 1;
            Some(Token::new(self.color, size))
        } else {
            None
        }
    }

    pub fn is_valid_place_from_inventory(
        &mut self,
        size: Size,
        board: &mut Board,
        x: usize,
        y: usize,
    ) -> bool {
        if self.inventory[size as usize] > 0 {
            if board.plate[y][x].is_stackable(Token::new(self.color, size)) {
                return true;
            }
        }
        false
    }

    pub fn place_from_inventory(
        &mut self,
        size: Size,
        board: &mut Board,
        x: usize,
        y: usize,
    ) -> bool {
        if self.is_valid_place_from_inventory(size, board, x, y) {
            self.inventory[size as usize] -= 1;
            board.plate[y][x].push_token(Token::new(self.color, size));
            true
        } else {
            false
        }
    }

    pub fn is_valid_swap_from_board(
        &mut self,
        board: &mut Board,
        x: usize,
        y: usize,
        x2: usize,
        y2: usize,
    ) -> bool {
        if x < GAME_SIZE && y < GAME_SIZE {
            if board.plate[y][x].get_outermost_token().is_some() {
                if board.plate[y2][x2].is_stackable(board.plate[y][x].get_outermost_token().unwrap()) {
                    return true;
                }
            }
        }
        false
    }


    pub fn swap_token_from_board(
        &mut self,
        board: &mut Board,
        x: usize,
        y: usize,
        x2: usize,
        y2: usize,
    ) -> bool {
        if self.is_valid_swap_from_board(board, x, y, x2, y2) {
            let token = board.plate[y][x].pop_outermost_token();
            board.plate[y2][x2].push_token(token);
            true
        } else {
            false
        }
    }
}

// Game processing  => Action + stauts = > New Status
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntEnum)]
pub enum ActionType {
    FromInventory = 0,
    FromBoard = 1,
}


#[derive(Clone, Copy, Debug)]
pub struct Action {
    pub action_type: ActionType,
    pub player: PlayerId,
    pub from_inventory: Option<Token>,
    pub from_xy: Option<[usize; 2]>,
    pub to_xy: Option<[usize; 2]>,
}

impl Action {
    pub fn new(action_type: ActionType, player: PlayerId, from_inventory: Option<Token>, from_xy: Option<[usize; 2]>, to_xy: Option<[usize; 2]>) -> Action {
        match action_type {
            ActionType::FromInventory => {
                Action {
                    action_type,
                    player,
                    from_inventory,
                    from_xy: None,
                    to_xy,
                }
            }
            ActionType::FromBoard => {
                Action {
                    action_type,
                    player,
                    from_inventory: None,
                    from_xy,
                    to_xy,
                }
            }
        }
    }
}


pub struct Gobblet {
    uid: String,
    board: Board,
    round_flag: PlayerId,
    players: [Player; 2],
}

impl Game<GAME_SIZE> for Gobblet {
    const NAME: &'static str = "Gobblet";
    const NUM_PLAYERS: usize = 2;
    // todo not for sure how long can it be.
    const MAX_TURNS: usize = 72;
    fn new() -> Gobblet {
        let mut rng = rand::thread_rng();
        let round_flag = match rng.gen_range(0..2) {
            0 => PlayerId::RED,
            1 => PlayerId::GREEN,
            _ => PlayerId::RED,
        };
        Gobblet {
            uid: Uuid::new_v4().to_string(),
            board: empty_board(),
            round_flag,
            players: [Player::new(round_flag), Player::new(!round_flag)],
        }
    }

    fn processing(&mut self) {
        while self.board.is_gameover().is_none() {
            self.board.display();
            let action = self.io_input();
            match action {
                Some(action) => {
                    self.processing_action(action);
                }
                None => {
                    println!("Invalid Action, Need to retry");
                }
            }
        }
        println!("end");
    }
    // keyboard input to trigger player action
    pub fn io_input(&mut self) -> Option<Action> {
        let option = self.get_option();
        match option {
            "a" => {
                println!("from inventory");
                let size = Self::get_size();
                let xy = Self::get_xy();
                Some(Action::new(ActionType::FromInventory, self.round_flag, self.players[self.round_flag as usize].get_token(Size::from(size)), None, Some([xy[0], xy[1]])))
            }
            "b" => {
                println!("from board");
                println!("from x y");
                let xy = Self::get_xy();
                let x = xy[0];
                let y = xy[1];
                println!("to x y");
                let xy = Self::get_xy();
                let x2 = xy[0];
                let y2 = xy[1];
                Some(Action::new(ActionType::FromBoard, self.round_flag, None, Some([x, y]), Some([x2, y2])))
            }
            _ => {
                println!("invalid input");
                None
            }
        }
    }
    fn step(&mut self, action: Action) {
        match action.action_type {
            ActionType::FromInventory => {
                if let Some(token) = action.from_inventory {
                    if self.players[action.player as usize].place_from_inventory(
                        token.size,
                        &mut self.board,
                        action.to_xy.unwrap()[0],
                        action.to_xy.unwrap()[1],
                    ) {
                        self.round_flag = !self.round_flag;
                    } else {
                        println!("invalid action");
                    }
                }
            }
            ActionType::FromBoard => {
                if self.players[action.player as usize].swap_token_from_board(
                    &mut self.board,
                    action.from_xy.unwrap()[0],
                    action.from_xy.unwrap()[1],
                    action.to_xy.unwrap()[0],
                    action.to_xy.unwrap()[1],
                ) {
                    self.round_flag = !self.round_flag;
                } else {
                    println!("invalid action");
                }
            }
        }
    }
    fn get_option(&self) -> &str {
        println!(
            "Player{:?}  a: take from invetory, b:board 2 board: ",
            self.round_flag
        );
        let mut in_str = String::new();
        io::stdin().read_line(&mut in_str).unwrap();
        let option = in_str.trim();
        match option {
            "a" => "a",
            "b" => "b",
            _ => self.get_option()
        }
    }

    fn get_size() -> Size {
        println!("size: 0:small, 1:medium, 2:large");
        let mut in_str = String::new();
        io::stdin().read_line(&mut in_str).unwrap();
        let size = in_str.trim();
        match size {
            "s" | "S" | "0" => Size::SMALL,
            "m" | "M" | "1" => Size::MID,
            "l" | "L" | "2" => Size::BIG,
            _ => {
                println!("invalid size");
                Self::get_size()
            }
        }
    }

    fn get_xy() -> [usize; 2] {
        println!("Enter the coordinate of the board");
        let mut in_str = String::new();
        io::stdin().read_line(&mut in_str).unwrap();
        let xy: Vec<usize> = in_str
            .trim()
            .split(&[' ', ','][..])
            .map(|s| s.parse().unwrap())
            .collect();
        let x = xy[0];
        let y = xy[1];
        [x, y]
    }

    pub fn parse_action(&mut self, action: Action) -> bool {
        match action.action_type {
            ActionType::FromInventory => {
                if let Some(token) = action.from_inventory {
                    if self.players[action.player as usize].place_from_inventory(
                        token.size,
                        &mut self.board,
                        action.to_xy.unwrap()[0],
                        action.to_xy.unwrap()[1],
                    ) {
                        return true;
                    }
                }
            }
            ActionType::FromBoard => {
                if self.players[action.player as usize].swap_token_from_board(
                    &mut self.board,
                    action.from_xy.unwrap()[0],
                    action.from_xy.unwrap()[1],
                    action.to_xy.unwrap()[0],
                    action.to_xy.unwrap()[1],
                ) {
                    return true;
                }
            }
        }
        false
    }
}

#[fixture]
fn end_board() -> Board {
    Board::new([
        [
            Block::new(vec![Token::new(PlayerId::RED, Size::BIG)]),
            Block::new(vec![Token::new(PlayerId::GREEN, Size::BIG)]),
            Block::new(vec![Token::new(PlayerId::GREEN, Size::BIG)]),
        ],
        [
            Block::new(vec![Token::new(PlayerId::GREEN, Size::BIG)]),
            Block::new(vec![Token::new(PlayerId::RED, Size::BIG)]),
            Block::new(vec![Token::new(PlayerId::GREEN, Size::BIG)]),
        ],
        [
            Block::new(vec![Token::new(PlayerId::GREEN, Size::SMALL)]),
            Block::new(vec![Token::new(PlayerId::GREEN, Size::SMALL)]),
            Block::new(vec![Token::new(PlayerId::RED, Size::SMALL)]),
        ],
    ])
}

#[fixture]
fn empty_board() -> Board {
    Board::new([
        [Block::new(vec![]), Block::new(vec![]), Block::new(vec![])],
        [Block::new(vec![]), Block::new(vec![]), Block::new(vec![])],
        [Block::new(vec![]), Block::new(vec![]), Block::new(vec![])],
    ])
}

#[fixture]
fn empty_player() -> Player {
    let mut p = Player::new(PlayerId::RED);
    p.inventory = [0, 0, 0];
    p
}

#[cfg(test)]
mod test_board {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[rstest]
    fn test_end_game(end_board: Board) {
        assert!(end_board.is_gameover() == Some(PlayerId::RED));
    }

    #[rstest]
    fn test_not_end_game(mut end_board: Board) {
        end_board.plate[1][1].pop_outermost_token();
        assert!(end_board.is_gameover().is_none());
    }

    #[rstest]
    fn test_display(mut end_board: Board) {
        end_board.display();
    }
}

#[cfg(test)]
mod test_player {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[rstest]
    fn test_place_from_inventory(
        mut empty_player: Player,
        mut empty_board: Board,
        mut end_board: Board,
    ) {
        let endb = &mut end_board;
        let emb = &mut empty_board;
        assert!(empty_player.place_from_inventory(Size::BIG, emb, 0, 0) == false);
        empty_player.inventory[2] += 1;
        assert!(empty_player.place_from_inventory(Size::BIG, emb, 0, 0) == true);
        empty_player.inventory[2] += 1;
        assert!(empty_player.place_from_inventory(Size::BIG, endb, 0, 0) == false);
    }

    #[rstest]
    fn test_place_from_board(
        mut empty_player: Player,
        mut empty_board: Board,
        mut end_board: Board,
    ) {
        let endb = &mut end_board;
        let emb = &mut empty_board;
        assert!(
            empty_player.swap_token_from_board(emb, 0, 0, 1, 1) == false,
            "should be false from empty to empty"
        );
        emb.plate[0][0].push_token(Token::new(PlayerId::RED, Size::BIG));
        assert!(empty_player.swap_token_from_board(emb, 0, 0, 1, 1) == true);
        assert!(emb.plate[0][0].tokens.is_empty());
        assert!(emb.plate[1][1].tokens.is_empty() == false);
    }
}



