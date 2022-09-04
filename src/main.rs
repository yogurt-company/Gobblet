use crate::game::Token;
use std::io;
use colored::*;
mod game;

fn main() {
    let mut game = game::Game::new();
    game.processing()
}
