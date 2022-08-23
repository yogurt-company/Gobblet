use crate::game::Token;

mod game;

fn main() {
    let t1:Token = game::Token::new(game::Color::RED, game::Size::BIG);
    let t2:Token = game::Token::new(game::Color::GREEN, game::Size::MID);
    let tokens = vec![t1,t2];
    let b = game::Block::new(tokens);
    println!("{:?}", b);
    println!("{:?}", b.pop_outermost_token());
}