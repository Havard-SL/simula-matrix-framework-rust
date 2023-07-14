#![allow(dead_code)]

mod common;

mod latin_square;

use latin_square::LatinSquare;
use latin_square::LatinStructure;
use latin_square::Permutation;
use latin_square::Sidedness;
use latin_square::sidedness;

mod abelian;

mod experiments;

mod affine_automorphism_table;

type AffineAutomorphism = (usize, usize, Sidedness);
type AllAffineAutomorphisms = (bool, Vec<AffineAutomorphism>);

pub trait LaTeX {
    fn latex(&self) -> String;
}

// Store max width?
pub trait ASCII {
    fn ascii(&self) -> Row<String>;

    fn width(&self) -> usize;

    fn height(&self) -> usize;
}

type Row<S> = Vec<S>;

// A table, where left[i] + right[i] becomes one row in the table.
// Split it up in order to make the abstraction cleaner and maybe slightly faster?
struct Table<L, R> {
    left: Vec<Row<L>>,
    right: Vec<Row<R>>,
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

impl LaTeX for LatinStructure {
    fn latex(&self) -> String {
        let text = match self {
            LatinStructure::Quasigroup => "Quasigroup",
            LatinStructure::Loop => "Loop",
            LatinStructure::Group => "Group",
            LatinStructure::Abelian => "Abelian",
        };

        text.to_string()
    }
}

struct LatinSquareClassification {
    class: LatinStructure,
    index: usize,
    square: LatinSquare,
    all_permutations_all_affine_automorphisms: Vec<AllAffineAutomorphisms>,
}

impl LatinSquareClassification {

    fn fingerprint(&self) -> usize {
        let mut fingerprint: usize = match self.class {
            LatinStructure::Quasigroup => 3,
            LatinStructure::Loop => 2,
            LatinStructure::Group => 1,
            LatinStructure::Abelian => 0,    
        };
    
        for (i, c) in self.all_permutations_all_affine_automorphisms.iter().enumerate() {
            if c.0 {
                fingerprint += 2_usize.pow((i + 2).try_into().unwrap());
            }
        }
    
        fingerprint
    }
}

impl LaTeX for Permutation {
    fn latex(&self) -> String {
        let mut text: String = "\\( \\begin{smallmatrix}\n".to_string();

        text.push_str(&self.0[0].to_string());

        for p in self.0.iter() {
            text.push_str(" & ");
            text.push_str(&p.to_string());
        }

        text.push_str("\n\\end{smallmatrix} \\)");

        text
    }
}

enum PermutationInformation {
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

enum SquareInformation {
    Class(LatinStructure),
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
            },
            Self::Index(index) => {
                text = "\\( s_{".to_string();
                text.push_str(&index.to_string());
                text.push_str("} \\)");
            },
            Self::Square(latin_square) => {
                text = latin_square.latex();
            },
            Self::None => {
                text = "".to_string();
            },
            // Self::AllAffineAutomorphisms(all_affine_automorphisms) => {
            //     text = all_affine_automorphisms.latex();
            // }
        }

        text
    }
}

impl LaTeX for LatinSquare {
    fn latex(&self) -> String {
        let mut text: String = "\\( \\begin{smallmatrix}\n".to_string();

        for row in self.0.iter() {
            text.push_str("    ");
            text.push_str(&row.first().unwrap().to_string());

            for v in row.iter().skip(1) {
                text.push_str(" & ");
                text.push_str(&v.to_string());
            }

            text.push_str("\\\\\n");
        }

        text.push_str("\\end{smallmatrix} \\)");

        text
    }
}

impl LaTeX for AffineAutomorphism {
    fn latex(&self) -> String {
        let mut text: String = "\\( ".to_string();
        
        match self.2 {
            Sidedness::Left => {
                text.push_str(&self.1.to_string());
                text.push_str(" + p_{");
                text.push_str(&self.0.to_string());
                text.push('}')
            }
            Sidedness::Right => {
                text.push_str("p_{");
                text.push_str(&self.0.to_string());
                text.push('}');
                text.push_str(" + ");
                text.push_str(&self.1.to_string());
            }
        };

        text.push_str(" \\)");

        text
    }
}

impl LaTeX for AllAffineAutomorphisms {
    fn latex(&self) -> String {
        let mut text: String = "\\begin{tabular}{c}\n    ".to_string();

        if self.0 {
            text.push('x');
        }

        for affine_automorphism in self.1.iter() {
            text.push_str("\\\\\\hline\n    ");
            text.push_str(&affine_automorphism.latex());

        }
        
        text.push_str("\n\\end{tabular}");

        text
    }
}

// fn calculate_fingerprint(classification: &LatinSquareClassification) -> usize {

//     let mut fingerprint: usize = match classification.class {
//         LatinStructure::Quasigroup => 3,
//         LatinStructure::Loop => 2,
//         LatinStructure::Group => 1,
//         LatinStructure::Abelian => 0,    
//     };

//     for (i, c) in classification.all_permutations_all_affine_automorphisms.iter().enumerate() {
//         if c.0 {
//             fingerprint += 2_usize.pow((i + 2).try_into().unwrap());
//         }
//     }

//     fingerprint
// }

fn classify_all_latin_squares(squares: &[LatinSquare], perms: &[Permutation]) -> Vec<LatinSquareClassification> {
    let mut result: Vec<LatinSquareClassification> = vec![];

    for (j, s) in squares.iter().enumerate() {
        let mut all_affine_automorphisms: Vec<AllAffineAutomorphisms> = vec![(false, vec![]); perms.len()];

        for (i, p) in perms.iter().enumerate() {
            let mut w = s.clone();
            w.apply_permutation(p.clone());

            if w == *s {
                all_affine_automorphisms[i].0 = true;

                for v in 0..squares[0].0.len() {

                    for side in sidedness::SIDES {
                        let affine_automorphism = s.addition_permutation(v, &side).compose(p);
                        let found_permutation = perms.iter().position(|x| x == &affine_automorphism).unwrap();
                        all_affine_automorphisms[found_permutation].1.push((i, v, side));
                    }
                }
            }
        }

        result.push(LatinSquareClassification {
            class: s.classify(), 
            index: j, 
            square: s.clone(), 
            all_permutations_all_affine_automorphisms: all_affine_automorphisms 
        });
    }

    result
}

fn create_table(rows: Vec<LatinSquareClassification>) -> Table<SquareInformation, PermutationInformation> {
    let mut left: Vec<Vec<SquareInformation>> = vec![vec![SquareInformation::None, SquareInformation::None, SquareInformation::None]];
    let mut right: Vec<Vec<PermutationInformation>> = vec![vec![]];
    
    for i in 0..rows[0].all_permutations_all_affine_automorphisms.len() {
        right[0].push(PermutationInformation::Index(i));
    }

    for (i, s) in rows.iter().enumerate() {
        left.push(vec![]);

        left[i + 1].push(SquareInformation::Square(s.square.clone()));
        left[i + 1].push(SquareInformation::Index(s.index));
        left[i + 1].push(SquareInformation::Class(s.class.clone()));

        right.push(vec![]);

        for affine_automorphisms in s.all_permutations_all_affine_automorphisms.iter() {
            right[i + 1].push(PermutationInformation::AllAffineAutomorphisms(affine_automorphisms.clone()));
        }
    }

    Table {left, right}
}

// TODO: Methods vs standalone functions.
fn main() {
    // Set the dimension of the Latin squares i generate.
    let n = 3;

    // Generate all the n by n latin squares.
    let squares = LatinSquare::generate_all(n);

    // Generate all the permutations on n elements.
    let perms = Permutation::generate_all(n);

    let mut classification = classify_all_latin_squares(&squares, &perms);
    
    classification.sort_by_cached_key(|x| x.fingerprint());

    let table = create_table(classification);

    let text = table.latex();

    println!("{text}");

}