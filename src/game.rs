use std::fmt;


// class Color(Flag):
//     RED = True
//     GREEN = False

pub enum Color {
    RED,
    GREEN,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match *self {
           Color::RED => write!(f, "R"),
           Color::GREEN => write!(f, "G")
       }
    }
}

// class Size(IntEnum):
//     BIG = 3
//     MID = 2
//     SMALL = 1


pub enum Size {
    BIG,
    MID,
    SMALL,
}
impl fmt::Display for Size {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match *self {
           Size::BIG => write!(f, "Big"),
           Size::MID => write!(f, "Mid"),
           Size::SMALL => write!(f, "Small")
       }
    }
}

pub struct Token {
    color : Color,
    size : Size,
}
impl Token {
    pub fn new(color: Color, size: Size) -> Token {
        Token {
            color: color,
            size: size
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "color: {}, size: {}", self.color, self.size)
    }
}