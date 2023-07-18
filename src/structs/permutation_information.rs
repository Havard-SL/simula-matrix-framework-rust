use std::fmt::Display;

use super::AllAffineAutomorphisms;
use super::Permutation;

use super::traits::SpreadsheetDisplay;

pub enum PermutationInformation {
    Permutation(Permutation),
    Index(usize),
    AllAffineAutomorphisms(AllAffineAutomorphisms),
}

impl Display for PermutationInformation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut text: String;

        match self {
            Self::Permutation(p) => text = p.to_string(),
            Self::Index(i) => {
                text = "p_".to_string();
                text.push_str(&i.to_string());
            }
            Self::AllAffineAutomorphisms(a) => text = a.spreadsheet_display(),
        };

        write!(f, "{}", text)
    }
}
