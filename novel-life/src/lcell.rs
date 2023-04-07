use std::fmt;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LCell {
    Alive,
    Dead,
}

impl fmt::Display for LCell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Alive => write!(f, "■"),
            Self::Dead => write!(f, " "),
        }
    }
}

impl From<char> for LCell {
    fn from(c: char) -> Self {
        match c {
            '#' => Self::Alive,
            '.' => Self::Dead,
            _ => panic!("unexpected cell string {}", c),
        }
    }
}
