use crate::game::Token;
use int_enum::IntEnum;
mod game;



fn main() {

    let t1:Token = game::Token::new(game::Color::RED, game::Size::SMALL);
    let t2:Token = game::Token::new(game::Color::GREEN, game::Size::MID);
    let tokens = vec![t1,t2];
    let mut b = game::Block::new(tokens);
    println!("{:?}", b.is_stackable(Token::new(game::Color::RED, game::Size::BIG)));
}