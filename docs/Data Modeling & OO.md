# Modeling & OO
嘿嘿嘿 終於要進入AI的領域了嗎? 不! 這裡的建模並不是Machine learning 的建模.
而是更單純的將外在世界透過一個合理的方式做[抽象化](https://zh.wikipedia.org/wiki/%E6%8A%BD%E8%B1%A1%E5%8C%96) 和 繼承.

抽象化概念:
> 主要是為了只保存和一特定目的有關的資訊。例如，將一個皮製的足球抽象化成一個球，只保留一般球的屬性和行為等資訊。相似地，亦可以將快樂抽象化成一種情緒，以減少其在情緒中所含的資訊量。

Object oriented development 經常會跟建模牽扯在一起. 而class又幾乎和OOD一起出現. 這裡就來介紹一下class的概念.
class希望把行為和status作為一個概念的罐頭封裝起來. 將來再使用的情境中, 只要把罐頭拿出來就可以使用裡面的行為和status.

透過合理的**抽象化** 和 **繼承** 可以讓我們的程式碼更加的可讀, 可維護, 可擴充.

-----
我知道本篇的讀者絕對不是希望來這裡複習class的概念. 但是對於來自於js 和 python的旅人, 如果談到OO卻跟他們說
> 不 本店沒有提供任何class 和繼承.

[官方cook book](https://doc.rust-lang.org/book/ch17-00-oop.html)也可以找到
>Many competing definitions describe what OOP is, and by some of these definitions Rust is object-oriented, but by others it is not.

就連官方都不確定Rust是不是OO. 但是我們可以確定的是, Rust以及其他現代程式語言都必須具備封裝概念/ 繼承/ 可擴抽的特性.


# 實戰建模囉 ^.<
## Enum
在定義有限選擇項目的時候可以透過 enum 去映照真實意義和詞. 很多時候我們在只有bool狀態下會直接使用 true/false 而不另外增加解釋意涵. 其實這對日後維護程式碼的人會有些許困擾.
```python
class Color(Flag):
    RED = True
    GREEN = False
class Size(IntEnum):
    BIG = 3
    MID = 2
    SMALL = 1
```
強烈的會感到rust非常**節制**的風格. 這邊enum必須透過derive來增加功能才能達成我們希望一個 int/bool enum應該有的功能. derive屬於 `macro` 我們會在之後的篇幅中介紹. 這邊先把他作為一個技能加強針來看待.
```rust
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntEnum)]
pub enum TokenColor {
    RED = 0,
    GREEN = 1,
}
impl Not for TokenColor {
    type Output = Self;
    fn not(self) -> Self::Output {
        match self {
            TokenColor::RED => TokenColor::GREEN,
            TokenColor::GREEN => TokenColor::RED,
        }
    }
}

#[repr(usize)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntEnum, PartialOrd)]
pub enum Size {
    BIG = 2,
    MID = 1,
    SMALL = 0,
}
```

## Struct
最基礎的能整合基礎data type 並且賦予儲存狀態的能力.

```python
class Token:
    def __init__(self, color: Color, size: Size):
        self.color = color
        self.size = size
    def __str__(self):
        return f'color: {self.color.name}, size: {self.size.name}'
```
python一般來說透過class封裝複數基礎資料/自建的struct(class). C, go , rust都是將行為和狀態分開封裝/實作. 所以沒有典型class, 某種可以一併繼承行為和狀態的語法. 在rust中只存在`has a` 不存在 `is a` 關係.
```rust
#[derive(Debug, Clone, Copy)]
pub struct Token {
    color: TokenColor,
    size: Size,
}
```
這邊要別注意rust在定義struct並沒有定義個別變數是 pub/private. mut/imu 完全會跟著母 struct的定義走.

## impl
rust 封裝行為是獨立開來的. 會有一個`impl` 關鍵字是很多語言沒有的. 

```python
# class Token:
#     def __init__(self, color: Color, size: Size):
#         self.color = color
#         self.size = size
    def __str__(self):
        return f'color: {self.color.name}, size: {self.size.name}'
```
```rust
impl Token {
    pub fn new(color: TokenColor, size: Size) -> Token {
        Token { color, size }
    }
    pub fn to_string(&self) -> String {
        match self.color {
            TokenColor::RED => match self.size {
                Size::BIG => "🔴".red().bold().to_string(),
                Size::MID => "🔴".red().to_string(),
                Size::SMALL => "🔴".red().dimmed().to_string(),
            },
            TokenColor::GREEN => match self.size {
                Size::BIG => "🟢".green().bold().to_string(),
                Size::MID => "🟢".green().to_string(),
                Size::SMALL => "🟢".green().dimmed().to_string(),
            },
        }
    }
}
```

# Generic / Trait
OO 其中一個重要意圖是增加程式復用性.

# mod

