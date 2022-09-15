use crate::game::Token;
use std::io;
use colored::*;
mod game;
mod algorithm;

fn main() {
    let a = [1, 2, 3, 4, 5];
    let summ:i32 = a.iter().filter(|&x| x % 2 == 0).map(|x| x*x).fold(0, |acc, x| acc + x);
    println!("{}", summ)
}


// 告訴我怎麼幹 A How to do it
// 告訴我幹什麼 B What to do it
