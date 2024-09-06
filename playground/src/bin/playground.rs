
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
    let t1 = Token {
        color: 123
    };
    let changed_token = update_token(t1, 2);
    println!("t1: {:?}, changed_token: {:?}", t1, changed_token);
}