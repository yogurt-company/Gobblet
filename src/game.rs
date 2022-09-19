use colored::*;
use int_enum::IntEnum;
use rand::Rng;
use rstest::*;
use std::io;
use std::ops::Not;
use uuid::Uuid;




#[repr(u8)]
#[derive(Clone, Copy, Debug,PartialEq, Eq, IntEnum)]
pub enum TokenColor {
    RED = 0,
    GREEN = 1,
}
impl Not for TokenColor {
    type Output = Self;
    fn not(self) -> Self::Output {
        match self {
            TokenColor::RED => TokenColor::GREEN,
            TokenColor::GREEN => TokenColor::RED,
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
    color: TokenColor,
    size: Size,
}

impl Token {
    pub fn new(color: TokenColor, size: Size) -> Token {
        Token { color, size }
    }
    pub fn to_string(&self) -> String {
        match self.color {
            TokenColor::RED => match self.size {
                Size::BIG => "🔴".red().bold().to_string(),
                Size::MID => "🔴".red().to_string(),
                Size::SMALL => "🔴".red().dimmed().to_string(),
            },
            TokenColor::GREEN => match self.size {
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
    // FP寫的
    fn pattern_check_fp(&self, pattern: [[usize; 2]; 3]) -> Option<TokenColor> {
        let result = pattern
            .iter()
            .map(|axis| self.plate[axis[0]][axis[1]].get_outermost_token())
            .filter(|t| t.is_some())
            .map(|t| t.unwrap().color)
            .fold([0, 0], |acc, c| match c {
                TokenColor::RED => [acc[0] + 1, acc[1]],
                TokenColor::GREEN => [acc[0], acc[1] + 1],
                _ => acc,
            });
        match result {
            [3, 0] => Some(TokenColor::RED),
            [0, 3] => Some(TokenColor::GREEN),
            _ => None,
        }
    }
    // TODO NxN genernalize
    pub fn is_gameover(&self) -> Option<TokenColor> {
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

    pub fn is_valid_take_from_board(&self, x: usize, y: usize) -> bool {
        if x < 3 && y < 3 {
            if self.plate[y][x].get_outermost_token().is_some() {
                return true;
            }
        }
        false
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
    color: TokenColor,
    inventory: [u8; 3],
}

impl Player {
    pub fn new(color: TokenColor) -> Player {
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

    pub fn place_from_inventory(
        &mut self,
        size: Size,
        board: &mut Board,
        x: usize,
        y: usize,
    ) -> bool {
        if self.inventory[size as usize] > 0 {
            if board.plate[y][x].push_token(Token::new(self.color, size)) {
                self.inventory[size as usize] -= 1;
                return true;
            }
        }
        false
    }

    pub fn place_from_board(
        &mut self,
        board: &mut Board,
        x: usize,
        y: usize,
        x2: usize,
        y2: usize,
    ) -> bool {
        if board.is_valid_take_from_board(x, y) {
            let token = board.plate[y][x].pop_outermost_token();
            if board.plate[y2][x2].push_token(token) {
                return true;
            }
            return false;
        }
        return false;
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
pub struct Action{
    pub action_type: ActionType,
    pub player: TokenColor,
    pub from_inventory: Option<Token>,
    pub from_xy: Option<[usize; 2]>,
    pub to_xy: Option<[usize; 2]>,
}

impl Action {
    pub fn new(action_type: ActionType, player: TokenColor, from_inventory: Option<Token>, from_xy: Option<[usize; 2]>, to_xy: Option<[usize; 2]>) -> Action {
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



pub struct Game {
    uid: String,
    board: Board,
    round_flag: TokenColor,
    players: [Player; 2],
}

impl Game {
    pub fn new() -> Game {
        let mut rng = rand::thread_rng();
        let round_flag = match rng.gen_range(0..2) {
            0 => TokenColor::RED,
            1 => TokenColor::GREEN,
            _ => TokenColor::RED,
        };
        Game {
            uid: Uuid::new_v4().to_string(),
            board: empty_board(),
            round_flag,
            players: [Player::new(round_flag), Player::new(!round_flag)],
        }
    }

    pub fn processing(&mut self) {
        while self.board.is_gameover().is_none() {
            let action = self.io_input();
            if self.parse_action(action) {
                self.round_flag = !self.round_flag;
            }
        }
        println!("end");
    }
    // keyboard input to trigger player action
    pub fn io_input(&mut self) -> Action {
        println!(
            "Player{:?}  a: take from invetory, b:board 2 board: ",
            self.round_flag
        );
        let mut in_str = String::new();
        io::stdin().read_line(&mut in_str).unwrap();
        let option = in_str.trim();
        match option {
            "a" => {
                println!("size: ,l/m/s");
                let mut in_str = String::new();
                io::stdin().read_line(&mut in_str).unwrap();
                let size = in_str.trim().parse::<u8>().unwrap();
                match size {
                    "l"=>
                }
                println!("x: ");
                let mut in_str = String::new();
                io::stdin().read_line(&mut in_str).unwrap();
                let x = in_str.trim().parse::<usize>().unwrap();
                println!("y: ");
                let mut in_str = String::new();
                io::stdin().read_line(&mut in_str).unwrap();
                let y = in_str.trim().parse::<usize>().unwrap();
                let action = Action::new(ActionType::FromInventory, self.round_flag, Some(Token::new(self.round_flag, Size::from(size))), None, Some([x, y]));
                action
            }
            "b" => {
                println!("x: ");
                let mut in_str = String::new();
                io::stdin().read_line(&mut in_str).unwrap();
                let x = in_str.trim().parse::<usize>().unwrap();
                println!("y: ");
                let mut in_str = String::new();
                io::stdin().read_line(&mut in_str).unwrap();
                let y = in_str.trim().parse::<usize>().unwrap();
                println!("x2: ");
                let mut in_str = String::new();
                io::stdin().read_line(&mut in_str).unwrap();
                let x2 = in_str.trim().parse::<usize>().unwrap();
                println!("y2: ");
                let mut in_str = String::new();
                io::stdin().read_line(&mut in_str).unwrap();
                let y2 = in_str.trim().parse::<usize>().unwrap();
                let action = Action::new(ActionType::FromBoard, self.round_flag, None, Some([x, y]), Some([x2, y2]));
                action
            }
            _ => {
                println!("invalid input");
                self.io_input()
            }
        }
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
                if self.players[action.player as usize].place_from_board(
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
            Block::new(vec![Token::new(TokenColor::RED, Size::BIG)]),
            Block::new(vec![Token::new(TokenColor::GREEN, Size::BIG)]),
            Block::new(vec![Token::new(TokenColor::GREEN, Size::BIG)]),
        ],
        [
            Block::new(vec![Token::new(TokenColor::GREEN, Size::BIG)]),
            Block::new(vec![Token::new(TokenColor::RED, Size::BIG)]),
            Block::new(vec![Token::new(TokenColor::GREEN, Size::BIG)]),
        ],
        [
            Block::new(vec![Token::new(TokenColor::GREEN, Size::SMALL)]),
            Block::new(vec![Token::new(TokenColor::GREEN, Size::SMALL)]),
            Block::new(vec![Token::new(TokenColor::RED, Size::SMALL)]),
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
    let mut p = Player::new(TokenColor::RED);
    p.inventory = [0, 0, 0];
    p
}

#[cfg(test)]
mod test_board {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[rstest]
    fn test_end_game(end_board: Board) {
        assert!(end_board.is_gameover() == Some(TokenColor::RED));
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
            empty_player.place_from_board(emb, 0, 0, 1, 1) == false,
            "should be false from empty to empty"
        );
        emb.plate[0][0].push_token(Token::new(TokenColor::RED, Size::BIG));
        assert!(empty_player.place_from_board(emb, 0, 0, 1, 1) == true);
        assert!(emb.plate[0][0].tokens.is_empty());
        assert!(emb.plate[1][1].tokens.is_empty() == false);
    }
}
