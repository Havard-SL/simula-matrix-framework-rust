use std::fmt::Display;

use super::AllAffineAutomorphisms;
use super::LatinSquare;
use super::LatinType;
use super::Permutation;

use super::traits::SpreadsheetDisplay;

pub enum SquareInformation {
    Class(LatinType),
    LatinSquareIndex(usize),
    LatinSquare(LatinSquare),
    Permutation(Permutation),
    PermutationIndex(usize),
    AllAffineAutomorphisms(AllAffineAutomorphisms),
    FingerprintIndex(usize),
    AutomorphismAndAffineSums((usize, usize)),
    None,
}

impl Display for SquareInformation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut text: String;

        match self {
            Self::Class(class) => {
                text = class.to_string();
            }
            Self::LatinSquareIndex(index) => {
                text = "s_".to_string();
                text.push_str(&index.to_string());
            }
            Self::LatinSquare(latin_square) => {
                text = latin_square.to_string();
            }
            Self::None => {
                text = "".to_string();
            }
            Self::Permutation(p) => text = p.to_string(),
            Self::PermutationIndex(i) => {
                text = "p_".to_string();
                text.push_str(&i.to_string());
            }
            Self::AllAffineAutomorphisms(a) => text = a.spreadsheet_display(),
            Self::FingerprintIndex(index) => {
                text = "F_".to_string();
                text.push_str(&index.to_string());
            }
            Self::AutomorphismAndAffineSums((aut, aff)) => {
                text = format!("Aut: {}\nAAut: {}", aut, aff);
            }
        }

        write!(f, "{}", text)
    }
}
