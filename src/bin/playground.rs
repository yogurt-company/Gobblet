use int_enum::IntEnum;


pub enum TokenColor {
    RED = 0,
    GREEN = 1,
}

pub enum Size {
    BIG = 2,
    MID = 1,
    SMALL = 0,
}

#[derive(Debug)]
pub struct Token {
    color: TokenColor,
    size: Size,
}


fn update_token(mut token:Token, render_color: TokenColor) -> Token {
    token.color = render_color;
    return token
}

fn main() {
    let t1 = Token {
        color: TokenColor::RED,
        size: Size::BIG,
    };
    let changed_token = update_token(t1, TokenColor::GREEN);
    println!("t1: {:?}, changed_token: {:?}", t1, changed_token);
}