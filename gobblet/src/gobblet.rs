use std::ops::Not;

use colored::*;
use int_enum::IntEnum;
use uuid::Uuid;
use synthesis::game::*;

const GAME_SIZE: usize = 3;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
    fn prev(&self) -> Self {
        self.next()
    }

    fn next(&self) -> Self {
        match self {
            PlayerId::RED => PlayerId::GREEN,
            PlayerId::GREEN => PlayerId::RED,
        }
    }
}

#[repr(usize)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntEnum, PartialOrd, Hash)]
pub enum Size {
    BIG = 2,
    MID = 1,
    SMALL = 0,
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
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

#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
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

#[derive(Debug, Default, Clone, Eq, Hash, PartialEq)]
pub struct Board {
    pub plate: Vec<Vec<Block>>,
}


impl Board {
    pub fn new() -> Self {
        let mut plate = Vec::with_capacity(GAME_SIZE);
        for _ in 0..GAME_SIZE {
            let mut row = Vec::with_capacity(GAME_SIZE);
            for _ in 0..GAME_SIZE {
                row.push(Block::default());
            }
            plate.push(row);
        }
        Board { plate }
    }
    pub fn to_features(&self) -> Vec<f32> {
        let mut features = Vec::new();

        for row in &self.plate {
            for block in row {
                if let Some(token) = block.get_outermost_token() {
                    features.extend(self.token_to_features(token));
                } else {
                    features.extend(vec![0.0; 6]);
                }
            }
        }
        features
    }

    fn token_to_features(&self, token: Token) -> Vec<f32> {
        let mut feature = vec![0.0; 6]; // 两个玩家，三种尺寸
        let index = match token.color {
            PlayerId::RED => 0,
            PlayerId::GREEN => 3,
        } + token.size.int_value();
        feature[index] = 1.0;
        feature
    }

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
    pub fn is_gameover(&self) -> Option<PlayerId> {
        // 检查行和列
        for i in 0..GAME_SIZE {
            // 检查行
            let row_positions: Vec<(usize, usize)> = (0..GAME_SIZE).map(|j| (i, j)).collect();
            if let Some(winner) = self.check_line(&row_positions) {
                return Some(winner);
            }
            // 检查列
            let col_positions: Vec<(usize, usize)> = (0..GAME_SIZE).map(|j| (j, i)).collect();
            if let Some(winner) = self.check_line(&col_positions) {
                return Some(winner);
            }
        }
        // 检查对角线
        let diag1_positions: Vec<(usize, usize)> = (0..GAME_SIZE).map(|i| (i, i)).collect();
        if let Some(winner) = self.check_line(&diag1_positions) {
            return Some(winner);
        }
        let diag2_positions: Vec<(usize, usize)> = (0..GAME_SIZE).map(|i| (i, GAME_SIZE - 1 - i)).collect();
        if let Some(winner) = self.check_line(&diag2_positions) {
            return Some(winner);
        }
        None
    }

    fn check_line(&self, positions: &[(usize, usize)]) -> Option<PlayerId> {
        let mut tokens = Vec::new();
        for &(x, y) in positions {
            if let Some(token) = self.plate[y][x].get_outermost_token() {
                tokens.push(token);
            } else {
                return None; // 如果任何位置为空，则不可能是获胜组合
            }
        }
        // 检查所有棋子是否属于同一玩家
        if tokens.iter().all(|t| t.color == PlayerId::RED) {
            return Some(PlayerId::RED);
        }
        if tokens.iter().all(|t| t.color == PlayerId::GREEN) {
            return Some(PlayerId::GREEN);
        }
        None
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntEnum, Hash)]
pub enum ActionType {
    FromInventory = 0,
    FromBoard = 1,
}


const ACTION_ID_MAX: usize = 3 * GAME_SIZE * GAME_SIZE + GAME_SIZE * GAME_SIZE * GAME_SIZE * GAME_SIZE;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Action {
    pub action_type: ActionType,
    pub player: PlayerId,
    pub from_inventory: Option<Token>,
    pub from_xy: Option<[usize; 2]>,
    pub to_xy: Option<[usize; 2]>,
}

// 为 Action 实现 Into<usize> 和 From<usize>
impl Into<usize> for Action {
    fn into(self) -> usize {
        match self.action_type {
            ActionType::FromInventory => {
                let size = self.from_inventory.unwrap().size as usize;
                let x = self.to_xy.unwrap()[0];
                let y = self.to_xy.unwrap()[1];
                size * GAME_SIZE * GAME_SIZE + y * GAME_SIZE + x
            }
            ActionType::FromBoard => {
                let from_x = self.from_xy.unwrap()[0];
                let from_y = self.from_xy.unwrap()[1];
                let to_x = self.to_xy.unwrap()[0];
                let to_y = self.to_xy.unwrap()[1];
                3 * GAME_SIZE * GAME_SIZE
                    + ((from_y * GAME_SIZE + from_x) * GAME_SIZE * GAME_SIZE)
                    + (to_y * GAME_SIZE + to_x)
            }
        }
    }
}

impl From<usize> for Action {
    fn from(value: usize) -> Self {
        if value < 3 * GAME_SIZE * GAME_SIZE {
            // FromInventory
            let size = value / (GAME_SIZE * GAME_SIZE);
            let idx = value % (GAME_SIZE * GAME_SIZE);
            let x = idx % GAME_SIZE;
            let y = idx / GAME_SIZE;
            Action {
                action_type: ActionType::FromInventory,
                player: PlayerId::RED, // 在实际使用中应根据当前玩家设置
                from_inventory: Some(Token::new(PlayerId::RED, Size::from_int(size).unwrap())),
                from_xy: None,
                to_xy: Some([x, y]),
            }
        } else {
            // FromBoard
            let adjusted_value = value - 3 * GAME_SIZE * GAME_SIZE;
            let from_idx = adjusted_value / (GAME_SIZE * GAME_SIZE);
            let to_idx = adjusted_value % (GAME_SIZE * GAME_SIZE);
            let from_x = from_idx % GAME_SIZE;
            let from_y = from_idx / GAME_SIZE;
            let to_x = to_idx % GAME_SIZE;
            let to_y = to_idx / GAME_SIZE;
            Action {
                action_type: ActionType::FromBoard,
                player: PlayerId::RED, // 在实际使用中应根据当前玩家设置
                from_inventory: None,
                from_xy: Some([from_x, from_y]),
                to_xy: Some([to_x, to_y]),
            }
        }
    }
}

pub struct ValidActions {
    game: Gobblet,
    action_id: usize,
}

impl ValidActions {
    pub fn new(game: &Gobblet) -> Self {
        ValidActions {
            game: game.clone(),
            action_id: 0,
        }
    }
}

impl Iterator for ValidActions {
    type Item = Action;

    fn next(&mut self) -> Option<Self::Item> {
        while self.action_id < ACTION_ID_MAX {
            let action = Action::from(self.action_id);
            self.action_id += 1;

            // 检查行动是否合法
            if self.game.is_action_valid(&action) {
                return Some(action);
            }
        }
        None
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Gobblet {
    uid: String,
    board: Board,
    player: PlayerId,
    players: [Player; 2],
    turn_count: usize,
}

impl Game<GAME_SIZE> for Gobblet {
    const NAME: &'static str = "Gobblet";
    const NUM_PLAYERS: usize = 2;
    const MAX_TURNS: usize = 72; // 根据游戏规则调整
    const DIMS: &'static [i64] = &[3, 3, 6]; // 根据游戏状态表示

    type PlayerId = PlayerId;
    type Action = Action;
    type ActionIterator = ValidActions;
    type Features = Vec<f32>;

    fn new() -> Self {
        Self {
            uid: Uuid::new_v4().to_string(),
            board: Board::new(),
            player: PlayerId::RED,
            players: [Player::new(PlayerId::RED), Player::new(PlayerId::GREEN)],
            turn_count: 0,
        }
    }

    fn player(&self) -> Self::PlayerId {
        self.player
    }

    fn is_over(&self) -> bool {
        self.board.is_gameover().is_some() || self.turn_count >= Self::MAX_TURNS
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
        ValidActions::new(self)
    }

    fn step(&mut self, action: &Self::Action) -> bool {
        if self.parse_action(*action) {
            self.player = !self.player;
            self.turn_count += 1;
            true
        } else {
            false
        }
    }

    fn features(&self) -> Self::Features {
        self.board.to_features()
    }

    fn print(&self) {
        self.board.display();
    }
}

impl Gobblet {
    pub fn is_action_valid(&self, action: &Action) -> bool {
        if action.player != self.player {
            return false;
        }

        match action.action_type {
            ActionType::FromInventory => {
                if let Some(token) = action.from_inventory {
                    self.players[action.player as usize].inventory[token.size as usize] > 0
                        && self.board.plate[action.to_xy.unwrap()[1]][action.to_xy.unwrap()[0]]
                        .is_stackable(token)
                } else {
                    false
                }
            }
            ActionType::FromBoard => {
                let from_block = &self.board.plate[action.from_xy.unwrap()[1]][action.from_xy.unwrap()[0]];
                if let Some(token) = from_block.get_outermost_token() {
                    token.color == action.player
                        && self.board.plate[action.to_xy.unwrap()[1]][action.to_xy.unwrap()[0]]
                        .is_stackable(token)
                        && !(action.from_xy.unwrap() == action.to_xy.unwrap())
                } else {
                    false
                }
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_creation() {
        let token_red_big = Token::new(PlayerId::RED, Size::BIG);
        assert_eq!(token_red_big.color, PlayerId::RED);
        assert_eq!(token_red_big.size, Size::BIG);

        let token_green_small = Token::new(PlayerId::GREEN, Size::SMALL);
        assert_eq!(token_green_small.color, PlayerId::GREEN);
        assert_eq!(token_green_small.size, Size::SMALL);
    }

    #[test]
    fn test_token_to_string() {
        let token_red_mid = Token::new(PlayerId::RED, Size::MID);
        let token_str = token_red_mid.to_string();
        // 由于 to_string 方法使用了终端颜色输出，这里可以检查是否包含预期的字符
        assert!(token_str.contains("🔴"));
    }


    #[test]
    fn test_block_creation() {
        let block = Block::default();
        assert!(block.tokens.is_empty());

        let token = Token::new(PlayerId::RED, Size::BIG);
        let block_with_token = Block::new(vec![token]);
        assert_eq!(block_with_token.tokens.len(), 1);
    }

    #[test]
    fn test_block_push_token() {
        let mut block = Block::default();
        let token_small = Token::new(PlayerId::RED, Size::SMALL);
        let token_mid = Token::new(PlayerId::GREEN, Size::MID);

        // 初始状态可以放置任何棋子
        assert!(block.push_token(token_small));
        assert_eq!(block.tokens.len(), 1);

        // 测试堆叠不可堆叠的棋子（尺寸相同或更小）
        let token_small_red = Token::new(PlayerId::GREEN, Size::SMALL);
        assert!(!block.push_token(token_small_red));
        assert_eq!(block.tokens.len(), 1); // 数量不变

        // 测试堆叠更大的棋子
        assert!(block.push_token(token_mid));
        assert_eq!(block.tokens.len(), 2);
        assert_eq!(block.get_outermost_token().unwrap(), token_mid);
    }

    #[test]
    fn test_block_is_stackable() {
        let mut block = Block::default();
        let token_small = Token::new(PlayerId::RED, Size::SMALL);
        let token_mid = Token::new(PlayerId::GREEN, Size::MID);

        assert!(block.is_stackable(token_small));
        block.push_token(token_small);

        // 相同或更小尺寸的棋子不可堆叠
        let token_small_red = Token::new(PlayerId::GREEN, Size::SMALL);
        assert!(!block.is_stackable(token_small_red));

        // 更大的棋子可堆叠
        assert!(block.is_stackable(token_mid));
    }

    #[test]
    fn test_block_get_outermost_token() {
        let mut block = Block::default();
        assert!(block.get_outermost_token().is_none());

        let token_small = Token::new(PlayerId::RED, Size::SMALL);
        block.push_token(token_small);
        assert_eq!(block.get_outermost_token().unwrap(), token_small);

        let token_big = Token::new(PlayerId::GREEN, Size::BIG);
        block.push_token(token_big);
        assert_eq!(block.get_outermost_token().unwrap(), token_big);
    }

    #[test]
    fn test_action_conversion() {
        // 测试从 usize 转换为 Action
        let action_id = 0;
        let action = Action::from(action_id);
        assert_eq!(action.action_type, ActionType::FromInventory);

        let action_id_back: usize = action.into();
        assert_eq!(action_id, action_id_back);
    }

    #[test]
    fn test_action_iterator() {
        let game = Gobblet::new();
        let actions: Vec<Action> = game.iter_actions().collect();

        // 检查是否生成了合法的行动
        assert!(!actions.is_empty());

        // 验证每个行动是否合法
        for action in actions {
            assert!(game.is_action_valid(&action));
        }
    }

    #[test]
    fn test_valid_actions_after_move() {
        let mut game = Gobblet::new();

        // 获取初始的合法行动数量
        let initial_actions: Vec<Action> = game.iter_actions().collect();
        let initial_action_count = initial_actions.len();

        // 执行一个行动
        let action = initial_actions[0];
        game.step(&action);

        // 获取新的合法行动数量
        let new_actions: Vec<Action> = game.iter_actions().collect();
        let new_action_count = new_actions.len();

        // 检查行动数量是否发生变化
        assert!(new_action_count <= initial_action_count);
    }


    #[test]
    fn test_player_creation() {
        let player_red = Player::new(PlayerId::RED);
        assert_eq!(player_red.color, PlayerId::RED);
        assert_eq!(player_red.inventory, [2, 2, 2]); // 每种尺寸2个棋子

        let player_green = Player::new(PlayerId::GREEN);
        assert_eq!(player_green.color, PlayerId::GREEN);
        assert_eq!(player_green.inventory, [2, 2, 2]);
    }

    #[test]
    fn test_player_get_token() {
        let mut player = Player::new(PlayerId::RED);

        // 获取一个小尺寸的棋子
        let token = player.get_token(Size::SMALL);
        assert!(token.is_some());
        assert_eq!(player.inventory[Size::SMALL as usize], 1);

        // 获取超过库存的棋子
        player.get_token(Size::SMALL);
        let token_none = player.get_token(Size::SMALL);
        assert!(token_none.is_none());
    }

    #[test]
    fn test_player_place_from_inventory() {
        let mut player = Player::new(PlayerId::RED);
        let mut board = Board::new();

        // 成功放置棋子
        let success = player.place_from_inventory(Size::SMALL, &mut board, 0, 0);
        assert!(success);
        assert_eq!(player.inventory[Size::SMALL as usize], 1);
        assert_eq!(
            board.plate[0][0].get_outermost_token().unwrap(),
            Token::new(PlayerId::RED, Size::SMALL)
        );

        // 在同一位置尝试放置更小或相同尺寸的棋子（应失败）
        let mut player_green = Player::new(PlayerId::GREEN);
        let fail = player_green.place_from_inventory(Size::SMALL, &mut board, 0, 0);
        assert!(!fail);

        // 放置更大的棋子（应成功）
        let success = player_green.place_from_inventory(Size::MID, &mut board, 0, 0);
        assert!(success);
        assert_eq!(
            board.plate[0][0].get_outermost_token().unwrap(),
            Token::new(PlayerId::GREEN, Size::MID)
        );
    }

    #[test]
    fn test_player_swap_token_from_board() {
        let mut player_red = Player::new(PlayerId::RED);
        let mut board = Board::new();

        // 玩家在 (0, 0) 放置一个小尺寸的棋子
        player_red.place_from_inventory(Size::SMALL, &mut board, 0, 0);

        // 尝试移动棋子到 (1, 1)
        let success = player_red.swap_token_from_board(&mut board, 0, 0, 1, 1);
        assert!(success);
        assert!(board.plate[0][0].get_outermost_token().is_none());
        assert_eq!(
            board.plate[1][1].get_outermost_token().unwrap(),
            Token::new(PlayerId::RED, Size::SMALL)
        );

        // 尝试移动其他玩家的棋子（应失败）
        let mut player_green = Player::new(PlayerId::GREEN);
        let fail = player_green.swap_token_from_board(&mut board, 1, 1, 2, 2);
        assert!(!fail);
    }
}