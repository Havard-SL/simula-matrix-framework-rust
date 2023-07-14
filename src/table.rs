use crate::latin_square::LatinType;

use super::latin_square::AllAffineAutomorphisms;
use super::LaTeX;
use super::LatinSquare;
use super::Permutation;

// A table, where left[i] + right[i] becomes one row in the table.
// Split it up in order to make the abstraction cleaner and maybe slightly faster?
pub struct Table<L, R> {
    pub left: Vec<Vec<L>>,
    pub right: Vec<Vec<R>>,
}

fn max_length<T>(rows: &[Vec<T>]) -> usize {
    let mut longest: usize = 0;

    for r in rows {
        if r.len() > longest {
            longest = r.len();
        }
    }

    longest
}

impl<L: LaTeX, R: LaTeX> LaTeX for Table<L, R> {
    // Assume: left and right have same length.
    fn latex(&self) -> String {
        let mut text = "".to_string();

        let left_length = max_length(&self.left);

        let n = left_length + self.right[0].len();

        text.push_str("\\begin{longtable}{|");
        text.push_str(&"c|".repeat(n));
        text.push_str("}\\hline\n");

        for (i, left_row) in self.left.iter().enumerate() {
            let right_row = &self.right[i];

            text.push_str("    ");

            let mut first_passed = false;

            for element in left_row.iter() {
                if first_passed {
                    text.push_str(" & ");
                } else {
                    first_passed = true;
                }
                text.push_str(&element.latex());
            }

            for element in right_row.iter() {
                if first_passed {
                    text.push_str(" & ");
                } else {
                    first_passed = true;
                }
                text.push_str(&element.latex());
            }

            text.push_str("\\\\\\hline\n");

            if i == 0 {
                text.push_str("\\endhead")
            }
        }

        text.push_str("\\end{longtable}");

        text
    }
}

pub enum PermutationInformation {
    Permutation(Permutation),
    Index(usize),
    AllAffineAutomorphisms(AllAffineAutomorphisms),
}

impl LaTeX for PermutationInformation {
    fn latex(&self) -> String {
        let mut text: String;

        match self {
            Self::Permutation(p) => text = p.latex(),
            Self::Index(i) => {
                text = "\\( p_{".to_string();
                text.push_str(&i.to_string());
                text.push_str("} \\)");
            }
            Self::AllAffineAutomorphisms(a) => text = a.latex(),
        };

        text
    }
}

pub enum SquareInformation {
    Class(LatinType),
    Index(usize),
    Square(LatinSquare),
    None,
}

impl LaTeX for SquareInformation {
    fn latex(&self) -> String {
        let mut text: String;

        match self {
            Self::Class(class) => {
                text = class.latex();
            }
            Self::Index(index) => {
                text = "\\( s_{".to_string();
                text.push_str(&index.to_string());
                text.push_str("} \\)");
            }
            Self::Square(latin_square) => {
                text = latin_square.latex();
            }
            Self::None => {
                text = "".to_string();
            } // Self::AllAffineAutomorphisms(all_affine_automorphisms) => {
              //     text = all_affine_automorphisms.latex();
              // }
        }

        text
    }
}
