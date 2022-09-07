# Modeling & OO
嘿嘿嘿 終於要進入AI的領域了嗎? 不! 這裡的建模並不是Machine learning 的建模.
而是更單純的將外在世界透過一個合理的方式做[抽象化](https://zh.wikipedia.org/wiki/%E6%8A%BD%E8%B1%A1%E5%8C%96) 和 繼承.

抽象化概念:
> 主要是為了只保存和一特定目的有關的資訊。例如，將一個皮製的足球抽象化成一個球，只保留一般球的屬性和行為等資訊。相似地，亦可以將快樂抽象化成一種情緒，以減少其在情緒中所含的資訊量。

Object oriented development 經常會跟建模牽扯在一起. 而class又幾乎和OOD一起出現. 這裡就來介紹一下class的概念.
class希望把行為和status作為一個概念的罐頭封裝起來. 將來再使用的情境中, 只要把罐頭拿出來就可以使用裡面的行為和status.

透過合理的**抽象化** 和 **繼承** 可以讓我們的程式碼更加的可讀, 可維護, 可擴充.

-----
我知道本篇的讀者絕對不是希望來這裡複習class的概念. 但是對於來自於js 和 python的旅人(或其他弱型態的coder), 如果談到OO卻跟他們說
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
class Token:
    def __init__(self, color: Color, size: Size):
        self.color = color
        self.size = size
###########vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv#############
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

# Trait
OO 其中一個重要意圖是增加程式復用性.
python, javascript的解題思路是透過多abstract一個層級作為parent class去規範共用的行為
```python
class Animal:
    def __init__(self, name: str):
        self.name = name
    def __str__(self):
        return f'{self.name}

class Dog(Animal):
    def bark(self):
        print('bark')
class Cat(Animal):
    def meow(self):
        print('meow')
```
`trait`作用也是達成同樣的目的. 因為**Cat** & **Dog** 都會有同樣的命名和print行為.抽出一層就可以達到程式碼復用性

在rust則換另外一個思考方向. 把需要child class變成一種input, 抽象成一個`trait` 並且讓**Cat** & **Dog**去實作他. 

```rust
trait Animal {
    // Associated function signature; `Self` refers to the implementor type.
    fn new(name: &'static str) -> Self;
    // Method signatures; these will return a string.
    fn name(&self) -> &'static str;
    // Traits can provide default method definitions.
    fn print(&self) {
        println!("{}",self.name());
    }
}
```


```rust
struct Dog { name: &'static str }

impl Animal for Dog {
    // `Self` is the implementor type: `Sheep`.
    fn new(name: &'static str) -> Dog {
        Dog { name: name }
    }
    fn bark(&self) {
        println!("bark");
    }
}
```



----
我們在實作gobblet其實沒有用到任何trait的語法. 這個語法對於擴展性來說是很重要的.

# Generic
Generic 基本上算是靜態語言獨有的特性. 但今天我們一起來思考一下為啥要有這個東東, 動態語言又是怎麼去解決這個問題. Generic最核心意涵就是某一個特定feature他要處理不同類別但是一樣的功能性.
拿一個最常見的案例至少最基本可以處理 數字和字串的case.

```python
class Point:
    def __init__(self, x, y):
        self.x = x
        self.y = y
    def __add__(self, other):
        return Point(self.x + other.x, self.y + other.y)

pt = Point(1,'2')
pt2 = Point(3,4)
```
這個很明顯的不能達成我們的邏輯. 但是如果我們把他改成這樣
```python
class Point:
    def __init__(self, x, y):
        self.x = x if isinstance(x, int) else int(x)
        self.y = y if isinstance(y, int) else int(y)
    def __add__(self, other):
        return Point(self.x + other.x, self.y + other.y)
```
在裡頭強制轉換型態成為一個可以 + 可以運作的 int型態基本上就可以處理多型態的問題. 但是這樣的寫法會讓我們的程式碼變得很難維護. 這個時候我們就可以用到Generic的概念了.

```rust
struct Point<T> {
    x: T,
    y: T,
}

impl Point<T>{
    fn add(&self, other: Point<T>) -> Point<T> {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    // let pt = Point { x: 5, y: 4.0 }; This will fail 
    let pt = Point { x: 5, y: 4 };
}
```
透過`T`做型態擴充, 但依舊保持型態檢查的特性. 這樣就可以達到我們的目的. 但是這樣的寫法還是有一個問題. 我們的`Point`只能處理`x`和`y`是同樣型態的情況. 但是如果我們想要處理`x`和`y`是不同型態的情況呢? 我們可以這樣寫

```rust
struct Point<T, U> {
    x: T,
    y: U,
}
impl Point<T,U> {
    fn add(&self, other: Point<T, U>) -> Point<T, U> {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
```


