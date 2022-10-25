use synthesis::game::*;
use std::ops::Not;

use colored::*;
use int_enum::IntEnum;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use uuid::Uuid;
use rstest::{fixture, rstest};


const GAME_SIZE: usize = 3;
const MAX_LAYERS: usize = GAME_SIZE;
const TOKEN_SIZE_TYPES: usize = GAME_SIZE;

#[repr(usize)]
#[derive(Clone, Copy, Debug, PartialEq, IntEnum, Eq, PartialOrd, Ord, std::hash::Hash)]
pub enum PlayerId {
    RED = 0,
    GREEN = 1,
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
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntEnum, EnumIter, PartialOrd, std::hash::Hash)]
pub enum Size {
    LARGE = 2,
    MID = 1,
    SMALL = 0,
}

#[derive(Debug, Clone, Copy, std::hash::Hash, PartialEq, Eq)]
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
                Size::LARGE => "L".red().bold().to_string(),
                Size::MID => "M".red().to_string(),
                Size::SMALL => "S".red().dimmed().to_string(),
            },
            PlayerId::GREEN => match self.size {
                Size::LARGE => "L".green().bold().to_string(),
                Size::MID => "M".green().to_string(),
                Size::SMALL => "S".green().dimmed().to_string(),
            },
        }
    }
}

#[derive(Debug, Default, Clone, std::hash::Hash, PartialEq, Eq)]
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

#[derive(Debug, Clone, std::hash::Hash, PartialEq, Eq)]
pub struct Board {
    pub plate: [[Block; 3]; 3],
}

impl Board {
    pub fn new(blocks: [[Block; 3]; 3]) -> Board {
        Board { plate: blocks }
    }
    // FP寫的
    fn pattern_check_fp(&self, pattern: [[usize; 2]; 3]) -> Option<PlayerId> {
        let result = pattern
            .iter()
            .map(|axis| self.plate[axis[0]][axis[1]].get_outermost_token())
            .filter(|t| t.is_some())
            .map(|t| t.unwrap().color)
            .fold([0, 0], |acc, c| match c {
                PlayerId::RED => [acc[0] + 1, acc[1]],
                PlayerId::GREEN => [acc[0], acc[1] + 1]
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
#[derive(Clone, Copy, Debug, Eq, PartialEq, std::hash::Hash)]
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
        board: &Board,
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
        board: &Board,
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

    pub fn list_all_valid_actions(&mut self, board: &Board) -> Vec<Action> {
        let mut actions = Vec::new();
        for y in 0..GAME_SIZE {
            for x in 0..GAME_SIZE {
                for y2 in 0..GAME_SIZE {
                    for x2 in 0..GAME_SIZE {
                        if self.is_valid_swap_from_board(board, x, y, x2, y2) {
                            actions.push(Action{
                                action_type: ActionType::SWAP,
                                from_inventory_size: None,
                                from_xy: Some([x, y]),
                                to_xy: Some([x2, y2]),
                            });
                        }
                    }
                }
                for size in Size::iter() {
                    if self.is_valid_place_from_inventory(size, board, x, y) {
                        actions.push(Action{
                                action_type: ActionType::FromInventory,
                                from_inventory_size: Some(size),
                                from_xy: None,
                                to_xy: Some([x, y]),
                            });
                    }
                }
            }
        }
        actions
    }
}


#[repr(usize)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntEnum, std::hash::Hash)]
pub enum ActionType {
    FromInventory = 0,
    SWAP = 1,
}


#[derive(Clone, Copy, Debug, std::hash::Hash, PartialEq, Eq)]
pub struct Action{
    pub action_type: ActionType,
    pub from_inventory_size: Option<Size>,
    pub from_xy: Option<[usize; 2]>,
    pub to_xy: Option<[usize; 2]>,
}

impl Action {
    pub fn new(action_type: ActionType, from_inventory: Option<Size>, from_xy: Option<[usize; 2]>, to_xy: Option<[usize; 2]>) -> Action {
        match action_type {
            ActionType::FromInventory => {
                Action {
                    action_type,
                    from_inventory_size: from_inventory,
                    from_xy: None,
                    to_xy,
                }
            }
            ActionType::SWAP => {
                Action {
                    action_type,
                    from_inventory_size: None,
                    from_xy,
                    to_xy,
                }
            }
        }
    }
}
// SWAP 9*9
// INV 3*9
// [SWAP |INV]
impl From<usize> for Action {
    fn from(i: usize) -> Action {
        const SWAP_RANGE: usize = (GAME_SIZE * GAME_SIZE) * (GAME_SIZE * GAME_SIZE);
        const SWAP_RANGE_UPPER:usize = SWAP_RANGE -1;
        const INV_RANGE: usize = SWAP_RANGE + (GAME_SIZE * GAME_SIZE * 3);
        return match i {
            0..=SWAP_RANGE_UPPER => Self {
                action_type: ActionType::SWAP,
                from_inventory_size: None,
                from_xy: Some([i % GAME_SIZE, (i / GAME_SIZE) % GAME_SIZE]),
                to_xy: Some([i / (GAME_SIZE * GAME_SIZE)% GAME_SIZE , (i / (GAME_SIZE * GAME_SIZE * GAME_SIZE)) % GAME_SIZE]),
            },
            SWAP_RANGE..=INV_RANGE => {
                let rest_i = i - SWAP_RANGE;
                Self {
                    action_type: ActionType::FromInventory,
                    from_inventory_size: Some(Size::from_int((rest_i/ (GAME_SIZE * GAME_SIZE * GAME_SIZE)) % GAME_SIZE).unwrap()),
                    from_xy: None,
                    to_xy: Some([(rest_i / (GAME_SIZE * GAME_SIZE )) % GAME_SIZE, rest_i % GAME_SIZE]),
                }
            },
            _ => panic!("Invalid action index"),
        };
    }
}

impl Into<usize> for Action {
        fn into(self) -> usize {
            match self.action_type {
                ActionType::SWAP => {
                    self.from_xy.unwrap()[0] + self.from_xy.unwrap()[1] * GAME_SIZE + self.to_xy.unwrap()[0] * GAME_SIZE * GAME_SIZE + self.to_xy.unwrap()[1] * GAME_SIZE * GAME_SIZE * GAME_SIZE
                }
                ActionType::FromInventory => {
                    self.from_inventory_size.unwrap().int_value() + self.to_xy.unwrap()[0] * 3 + self.to_xy.unwrap()[1] * 3 * GAME_SIZE + (GAME_SIZE * GAME_SIZE * 3)
                }
            }
        }
    }



#[derive(Debug, std::hash::Hash, Clone, PartialEq, Eq)]
pub struct Gobblet {
    uid: String,
    board: Board,
    round_flag: PlayerId,
    players: [Player; 2],
}

impl Gobblet {
    pub fn new() -> Gobblet {
        Gobblet {
            uid: Uuid::new_v4().to_string(),
            board: empty_board(),
            round_flag: PlayerId::RED,
            players: [Player::new(PlayerId::RED), Player::new(PlayerId::GREEN)],
        }
    }

    // pub fn processing(&mut self) {
    //     while self.board.is_gameover().is_none() {
    //         self.board.display();
    //         let action = self.io_input();
    //         match action {
    //             Some(action) => {
    //                 self.processing_action(action);
    //             }
    //             None => {
    //                 println!("Invalid Action, Need to retry");
    //             }
    //         }
    //     }
    //     println!("end");
    // }
    // keyboard input to trigger player action
    // pub fn io_input(&mut self) -> Option<Action>{
    //     let option = self.get_option();
    //     match option {
    //         "a" => {
    //             println!("from inventory");
    //             let size = Self::get_size();
    //             let xy= Self::get_xy();
    //             Some(Action::new(ActionType::FromInventory, Some(size), None, Some([xy[0], xy[1]])))
    //         }
    //         "b" => {
    //             println!("from board");
    //             println!("from x y");
    //             let xy = Self::get_xy();
    //             let x = xy[0];
    //             let y = xy[1];
    //             println!("to x y");
    //             let xy = Self::get_xy();
    //             let x2 = xy[0];
    //             let y2 = xy[1];
    //             Some(Action::new(ActionType::SWAP, None, Some([x, y]), Some([x2, y2])))
    //         }
    //         _ => {
    //             println!("invalid input");
    //             None
    //         }
    //     }
    // }
    fn processing_action(&mut self, action: Action) {
        match action.action_type {
            ActionType::FromInventory => {
                if let Some(token) = action.from_inventory_size {
                    if self.players[self.round_flag as usize].place_from_inventory(
                        action.from_inventory_size.unwrap(),
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
            ActionType::SWAP => {
                if self.players[self.round_flag as usize].swap_token_from_board(
                    &mut self.board,
                    action.from_xy.unwrap()[0],
                    action.from_xy.unwrap()[1],
                    action.to_xy.unwrap()[0],
                    action.to_xy.unwrap()[1],
                ) {
                    self.round_flag = !self.round_flag;
                }
                else {
                    println!("invalid action");
                }
            }
        }
    }
    // fn get_option(&self) -> &str {
    //     println!(
    //         "Player{:?}  a: take from invetory, b:board 2 board: ",
    //         self.round_flag
    //     );
    //     let mut in_str = String::new();
    //     io::stdin().read_line(&mut in_str).unwrap();
    //     let option = in_str.trim();
    //     match option {
    //         "a" => "a",
    //         "b" => "b",
    //         _ => self.get_option()
    //     }
    // }
    //
    // fn get_size() -> Size {
    //     println!("size: 0:small, 1:medium, 2:large");
    //     let mut in_str = String::new();
    //     io::stdin().read_line(&mut in_str).unwrap();
    //     let size = in_str.trim();
    //     match size {
    //         "s" | "S" | "0" => Size::SMALL,
    //         "m" | "M" | "1" => Size::MID,
    //         "l" | "L" | "2" => Size::LARGE,
    //         _ => {
    //             println!("invalid size");
    //             Self::get_size()
    //         }
    //     }
    // }
    //
    // fn get_xy() -> [usize; 2] {
    //     println!("Enter the coordinate of the board");
    //     let mut in_str = String::new();
    //     io::stdin().read_line(&mut in_str).unwrap();
    //     let xy: Vec<usize> = in_str
    //         .trim()
    //         .split(&[' ',','][..])
    //         .map(|s| s.parse().unwrap())
    //         .collect();
    //     let x = xy[0];
    //     let y = xy[1];
    //     [x, y]
    // }
}



pub struct ActionIterator {
    game: Gobblet,
    iter_count: usize,
}



impl Iterator for ActionIterator {
    type Item = Action;
    // todo bug is here, iterall valid is not correct
    fn next(&mut self) -> Option<Self::Item> {
        let valid_actions = self.game.players[self.game.round_flag as usize].list_all_valid_actions(&self.game.board);
        if self.iter_count < valid_actions.len() {
            let action = valid_actions[self.iter_count];
            self.iter_count += 1;
            Some(action)
        } else {
            None
        }
    }
}

impl Game<108> for Gobblet {
    const NAME: &'static str = "Gobblet";
    const NUM_PLAYERS: usize = 2;
    const MAX_TURNS: usize = 60;//TODO not for sure how long the game will last

    type PlayerId = PlayerId;
    type Action = Action;
    type ActionIterator = ActionIterator;

    fn new() -> Self {
        Self::new()
    }

    fn player(&self) -> Self::PlayerId {
        self.round_flag
    }

    fn is_over(&self) -> bool {
        self.board.is_gameover().is_some()
    }

    fn reward(&self, player_id: Self::PlayerId) -> f32 {

        match self.board.is_gameover() {
            Some(winner) => {
                if winner == player_id {
                    1.0
                } else {
                    -1.0
                }
            }
            None => 0.0,
        }
    }
    fn iter_actions(&self) -> Self::ActionIterator {
        ActionIterator {
            game: self.clone(),
            iter_count: 0,
        }
    }

    fn step(&mut self, action: &Self::Action) -> bool {
        self.processing_action(action.clone());
        self.is_over()
    }

    //Define all status of the game
    const DIMS: &'static [i64] = &[3,3,3,3];
    type Features = [[[[f32;TOKEN_SIZE_TYPES]; MAX_LAYERS]; GAME_SIZE]; GAME_SIZE];
    fn features(&self) -> Self::Features {
        let mut s = Self::Features::default();
        for i in 0..GAME_SIZE {
            for j in 0..GAME_SIZE {
                for (layer, token) in self.board.plate[i][j].tokens.iter().rev().enumerate(){
                    match token{
                        Token{color:token_color,size:token_size} => s[i][j][layer][token_size.int_value()] =
                            match token_color{
                            PlayerId::RED => 1.0,
                            PlayerId::GREEN => -1.0}
                    }
                }
            }
        }
        s
    }

    fn print(&self) {
        if self.is_over() {
            println!("{:?} won", self.board.is_gameover().unwrap());
        } else {
            println!("{:?} to play", self.player());
            //TODO print all avalible action?
        }
        self.board.display();
    }
}


#[fixture]
fn end_board() -> Board {
    Board::new([
        [
            Block::new(vec![Token::new(PlayerId::RED, Size::LARGE)]),
            Block::new(vec![Token::new(PlayerId::GREEN, Size::LARGE)]),
            Block::new(vec![Token::new(PlayerId::GREEN, Size::LARGE)]),
        ],
        [
            Block::new(vec![Token::new(PlayerId::GREEN, Size::LARGE)]),
            Block::new(vec![Token::new(PlayerId::RED, Size::LARGE)]),
            Block::new(vec![Token::new(PlayerId::GREEN, Size::LARGE)]),
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

#[fixture]
fn full_player() -> Player {
    let mut p = Player::new(PlayerId::RED);
    p.inventory = [2, 2, 2];
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
mod test_game {
    use super::*;

    #[rstest]
    fn test_features(mut end_board: Board) {
        let mut gobblet = Gobblet::new();
        gobblet.board = end_board;
        assert_eq!(gobblet.features(), [[[[0.0, 0.0, 1.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0]], [[0.0, 0.0, -1.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0]], [[0.0, 0.0, -1.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0]]], [[[0.0, 0.0, -1.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0]], [[0.0, 0.0, 1.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0]], [[0.0, 0.0, -1.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0]]], [[[-1.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0]], [[-1.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0]], [[1.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0]]]]);
    }
    #[rstest]
    fn test_print(mut end_board: Board) {
        let mut gobblet = Gobblet::new();
        gobblet.board = end_board;
        gobblet.print();
    }

    #[rstest]
    fn test_player() {
        let mut gobblet = Gobblet::new();
        let player = gobblet.player();
        assert_eq!(player, PlayerId::RED);
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
        assert!(empty_player.place_from_inventory(Size::LARGE, emb, 0, 0) == false);
        empty_player.inventory[2] += 1;
        assert!(empty_player.place_from_inventory(Size::LARGE, emb, 0, 0) == true);
        empty_player.inventory[2] += 1;
        assert!(empty_player.place_from_inventory(Size::LARGE, endb, 0, 0) == false);
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
        emb.plate[0][0].push_token(Token::new(PlayerId::RED, Size::LARGE));
        assert!(empty_player.swap_token_from_board(emb, 0, 0, 1, 1) == true);
        assert!(emb.plate[0][0].tokens.is_empty());
        assert!(emb.plate[1][1].tokens.is_empty() == false);
    }

    #[rstest]
    fn test_list_all_actions(
        mut empty_player: Player,
        mut full_player: Player,
        mut empty_board: Board,
    ){
        let emb = &mut empty_board;
        const ALL_POSSIBLE_FROM_INVENTOR_ACTIONS:usize = GAME_SIZE*GAME_SIZE*3;
        let actions_list = full_player.list_all_valid_actions(emb);
        assert_eq!(actions_list.len(),ALL_POSSIBLE_FROM_INVENTOR_ACTIONS);
        let actions_list = empty_player.list_all_valid_actions(emb);
        assert_eq!(actions_list.len(),0);

    }
    #[rstest]
    fn test_action_from(){
        let mut dut_action = Action::from(0);
        assert_eq!(dut_action,Action{action_type:ActionType::SWAP,
            from_inventory_size: None,
            from_xy:Some([0;2]),to_xy:Some([0;2])});
        dut_action = Action::from(9);
        assert_eq!(dut_action,Action{action_type:ActionType::SWAP,
            from_inventory_size: None,
            from_xy:Some([0,0]),to_xy:Some([1,0])});
        dut_action = Action::from(81);
        assert_eq!(dut_action,Action{action_type:ActionType::FromInventory,
            from_inventory_size: Some(Size::SMALL),
            from_xy:None,to_xy:Some([0,0])});
    }

}



