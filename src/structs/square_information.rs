use std::fmt::Display;

use super::LatinSquare;
use super::LatinType;

pub enum SquareInformation {
    Class(LatinType),
    Index(usize),
    Square(LatinSquare),
    None,
}

impl Display for SquareInformation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut text: String;

        match self {
            Self::Class(class) => {
                text = class.to_string();
            }
            Self::Index(index) => {
                text = "s_".to_string();
                text.push_str(&index.to_string());
            }
            Self::Square(latin_square) => {
                text = latin_square.to_string();
            }
            Self::None => {
                text = "".to_string();
            }
        }

        write!(f, "{}", text)
    }
}
