// #[derive(Debug,Copy)]
// pub enum TokenColor {
//     RED = 0,
//     GREEN = 1,
// }
// #[derive(Debug,Copy)]
// pub enum Size {
//     BIG = 2,
//     MID = 1,
//     SMALL = 0,
// }

#[derive(Debug,Copy)]
pub struct Token {
    color: i32,
    // size: Size,
}


fn update_token(mut token:Token, render_color: i32) -> Token {
    token.color = render_color;
    return token
}

fn main() {
    // let t1 = Token {
    //     color: TokenColor::RED,
    //     size: Size::BIG,
    // };
    // let changed_token = update_token(t1, TokenColor::GREEN);
    // println!("t1: {:?}, changed_token: {:?}", t1, changed_token);
    let t1 = Token {
        color: 123
    };
    let changed_token = update_token(t1, 2);
    println!("t1: {:?}, changed_token: {:?}", t1, changed_token);
}