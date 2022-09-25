# Derive Conti.


- ~~Clone, to create T from &T via a copy.~~
- ~~Copy, to give a type 'copy semantics' instead of 'move semantics'.~~
- ~~Debug, to format a value using the {:?} formatter.~~
- Default, to create an empty instance of a data type.
- PartialEq 與 Eq
- Hash, to compute a hash from &T.

承上篇. 
> 我們嘗試實現一個函數來更新Token的狀態, 一個隨意自定義的data structure.

# PartialEq 與 Eq

但這次我們想要去比較兩個Token是否相等. Aka 我們想要 tokenA == TokenB. 這時候我們就需要用到`PartialEq` 與 `Eq` 這兩個trait.
撇除學院派的 partial eq concept(你可以輕易的google到這個資料).

```rust
#[derive(Debug,Copy)]
pub enum TokenColor {
    RED ,
    GREEN
}
#[derive(Debug,Copy)]
pub enum Size {
    BIG,
    MID
}
#[derive(Debug,Copy)]
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
```