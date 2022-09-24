# Derive
rust一路寫來都會給人一種非常節制且保守的風格, 能不多給功能就不多給功能.這幾個好夥伴幾乎可以說是幾乎是在寫code一開始就必須要用到的.


- Clone, to create T from &T via a copy.
- Copy, to give a type 'copy semantics' instead of 'move semantics'.
- Hash, to compute a hash from &T.
- Default, to create an empty instance of a data type.
- Debug, to format a value using the {:?} formatter.



# Hash
當你需要一些共通功能的時候必需要,舉個實際的例子.在筆者實現遊戲的過程中. 會遇上需要比對兩個棋盤是否一致的問題

```rust

#[repr(u8)]
#[derive(Clone, Copy, Debug,PartialEq, Eq, IntEnum)]
pub enum TokenColor {
    RED = 0,
    GREEN = 1,
}

#[repr(usize)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntEnum, PartialOrd)]
pub enum Size {
    BIG = 2,
    MID = 1,
    SMALL = 0,
}


pub struct Token {
    color: TokenColor,
    size: Size,
}


pub struct Block {
    tokens: Vec<Token>,
}

pub struct Board {
    pub plate: [[Block; 3]; 3],
}
```



# 再訪 Clone & Copy