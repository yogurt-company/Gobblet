// class Color(Flag):
//     RED = True
//     GREEN = False
//
// class Size(IntEnum):
//     BIG = 3
//     MID = 2
//     SMALL = 1
//
//
// class Token:
//     def __init__(self, color: Color, size: Size):
//         self.color = color
//         self.size = size
//     def __str__(self):
//         return f'color: {self.color.name}, size: {self.size.name}'
[!derive!()]
enum Color {
    Red,
    Green,
}

enum Size {
    Big,
    Mid,
    Small,
}

struct Token {
    color: Color,
    size: Size,
}

impl Token {
    fn new(color: Color, size: Size) -> Self {
        Self { color, size }
    }
    fn is_
}