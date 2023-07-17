use std::fmt::Display;

use indicatif::ProgressBar;

pub mod permutation;
pub use permutation::Permutation;

pub use self::sidedness::Sidedness;

pub mod sidedness;

pub mod bits;
pub use bits::Bits;

use super::LaTeX;

use super::table::PermutationInformation;
use super::table::SquareInformation;
use super::table::Table;

#[derive(Debug, Clone, PartialEq)]
struct PartialLatinSquare(Vec<Vec<usize>>);

// Takes a partial latin square, and recursively creates the next partial latin squares,
// until there are only full latin squares left.
// TODO: May run faster if split up the cases where col = n or not?
fn latin_square_recursion(n: usize, partial: PartialLatinSquare) -> Vec<LatinSquare> {
    let mut result: Vec<LatinSquare> = vec![];

    let last = partial.0.last().unwrap();

    let mut col = last.len();
    let mut row = partial.0.len() - 1;

    if col == n {
        col = 0;
        row += 1;
    }

    'val: for i in 0..n {
        // Check if i exists on current row.
        if let Some(l) = partial.0.get(row) {
            if l.contains(&i) {
                continue 'val;
            }
        }

        // Check if i exist on column.
        for r in partial.0.iter().take(row) {
            if r[col] == i {
                continue 'val;
            }
        }

        // Add i to the partial latin square.
        let mut p = partial.0.clone();

        if col == 0 {
            p.push(vec![i]);
        } else {
            p.last_mut().unwrap().push(i);

            if row == n - 1 && col == n - 1 {
                return vec![LatinSquare(p)];
            }
        }

        let p = PartialLatinSquare(p);

        // Recursive call.
        result.append(&mut latin_square_recursion(n, p));
    }

    result
}

// The different classes that a latin square can belong to.
#[derive(Debug, Clone, PartialEq)]
pub enum LatinStructure {
    Quasigroup,
    Loop,
    Group,
    Abelian,
}

// Represented as a vector of the rows of the latin square, where the rows are vectors of usize.
// Always non-empty, square, and satisfies the latin square property.
#[derive(Debug, Clone, PartialEq)]
pub struct LatinSquare(pub Vec<Vec<usize>>);

impl LatinSquare {
    // Prints the Latin square in a pretty way.
    pub fn print(&self) {
        let length = self.0.len();

        let n = length * 3 + 3;
        let border = "-".repeat(n);

        let newline = "\n";

        let mut super_string: String = "".to_string();

        super_string.push_str(&border);
        super_string.push_str(newline);

        for row in 0..length {
            let mut string = "|".to_string();
            for column in 0..length {
                let val = self.0[row][column];
                if val < 10 {
                    string.push_str("  ");
                    string.push_str(&val.to_string());
                } else {
                    string.push(' ');
                    string.push_str(&val.to_string());
                }
            }
            string.push_str(" |");

            super_string.push_str(&string);
            super_string.push_str(newline);
        }
        super_string.push_str(&border);

        println!("{}", super_string)
    }

    // Generates all latin squares. No fixed identity.
    pub fn generate_all(n: usize) -> Vec<LatinSquare> {
        let mut result: Vec<LatinSquare> = vec![];
        let bar = ProgressBar::new(n as u64);

        for i in 0..n {
            bar.inc(1);
            result.append(&mut latin_square_recursion(
                n,
                PartialLatinSquare(vec![vec![i]]),
            ));
        }

        result
    }

    // Applies a permutation to a latin square. I.e. gives the conjugacy of the latin square.
    pub fn apply_permutation(&mut self, mut p: Permutation) {
        // Apply the permutation to every element in the latin square.
        for row in self.0.iter_mut() {
            for column in row.iter_mut() {
                *column = p.0[*column]
            }
        }

        // Permutes the rows and columns of the latin square by the inverse of the permutation
        // in a pairwise way, by turning the permutation into transmutations.
        let length = p.0.len();

        for i in 0..length {
            let j = p.0.iter().position(|&x| x == i).unwrap();

            if i != j {
                self.0.swap(i, j);

                for row in self.0.iter_mut() {
                    row.swap(i, j)
                }

                p.0.swap(i, j);
            }
        }
    }

    fn left_identity(&self) -> bool {
        // Check if it contains a right-identity
        let standard: Vec<usize> = (0..self.0.len()).collect();

        self.0.contains(&standard)
    }

    fn right_identity(&self) -> bool {
        let mut standard: Vec<usize> = (0..self.0.len()).collect();

        let column = self.0[0].iter().position(|&x| x == 0).unwrap();

        for row in self.0.iter().rev() {
            let c = standard.pop().unwrap();

            if c != row[column] {
                return false;
            }
        }

        true
    }

    fn commutative(&self) -> bool {
        for a in 1..self.0.len() {
            for b in 0..a {
                if self.0[a][b] != self.0[b][a] {
                    return false;
                }
            }
        }

        true
    }

    fn associative(&self) -> bool {
        todo!()
    }

    // Classifies the Latin square as a quasigroup, loop, group or abelian group.
    pub fn classify(&self) -> LatinStructure {
        if !self.left_identity() {
            return LatinStructure::Quasigroup;
        }

        // Check if it contains a left-identity
        if !self.right_identity() {
            return LatinStructure::Quasigroup;
        }

        // Check if associative
        let a: Vec<usize>;

        let column = self.0[0].iter().position(|&x| x == 0).unwrap();

        if column == 0 {
            a = (1..self.0.len()).collect()
        } else if column == self.0.len() - 1 {
            a = (0..(self.0.len() - 1)).collect()
        } else {
            let mut a_1: Vec<usize> = (0..column).collect();
            a_1.append(&mut ((column + 1)..self.0.len()).collect());

            a = a_1;
        };

        let mut associativity_triplets: Vec<(usize, usize, usize)> = vec![];

        for x in &a {
            for y in &a {
                for z in &a {
                    associativity_triplets.push((*x, *y, *z));
                }
            }
        }

        for (a, b, c) in associativity_triplets {
            let left = self.0[a][self.0[b][c]];
            let right = self.0[self.0[a][b]][c];

            if left != right {
                return LatinStructure::Loop;
            }
        }

        // Check if symmetric
        if !self.commutative() {
            return LatinStructure::Group;
        }

        LatinStructure::Abelian
    }

    pub fn classify_structure(&self) -> LatinType {
        let class = self.classify();

        // Left-id, right-id, commutative
        let flags: (bool, bool, bool) = match class {
            LatinStructure::Abelian => (true, true, true),
            LatinStructure::Group => (true, true, false),
            LatinStructure::Loop => {
                if self.commutative() {
                    (true, true, true)
                } else {
                    (true, true, false)
                }
            }
            LatinStructure::Quasigroup => {
                if self.commutative() {
                    (false, false, true)
                } else if self.left_identity() {
                    (true, false, false)
                } else if self.right_identity() {
                    (false, true, false)
                } else {
                    (false, false, false)
                }
            }
        };

        LatinType {
            commutative: flags.2,
            left_identity: flags.0,
            right_identity: flags.1,
            structure: class,
        }
    }

    pub fn addition_permutation(&self, v: usize, side: &Sidedness) -> Permutation {
        let mut result: Vec<usize> = vec![];

        for i in 0..self.0.len() {
            let r = match side {
                Sidedness::Left => self.0[v][i],
                Sidedness::Right => self.0[i][v],
            };
            result.push(r);
        }

        Permutation(result)
    }
}

impl Display for LatinStructure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            LatinStructure::Quasigroup => "Quasigroup",
            LatinStructure::Loop => "Loop",
            LatinStructure::Group => "Group",
            LatinStructure::Abelian => "Abelian",
        };

        write!(f, "{}", text)
    }
}

impl LaTeX for LatinStructure {
    fn latex(&self) -> String {
        self.to_string()
    }
}

#[derive(Clone)]
pub struct LatinType {
    structure: LatinStructure,
    left_identity: bool,
    right_identity: bool,
    commutative: bool,
}

impl Display for LatinType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut text: String = "".to_string();

        text.push_str(&self.structure.to_string());

        text.push_str(match self.structure {
            LatinStructure::Loop => {
                if self.commutative {
                    "\nCommutative"
                } else {
                    ""
                }
            }
            LatinStructure::Quasigroup => {
                if self.commutative {
                    "\nCommutative"
                } else if self.left_identity {
                    "\nLeft Identity"
                } else if self.right_identity {
                    "\nRight Identity"
                } else {
                    ""
                }
            }
            _ => "",
        });
        
        write!(f, "{}", &text)
    }
}

impl LaTeX for LatinType {
    fn latex(&self) -> String {
        let mut text: String = "".to_string();

        let mut additional_rows: Vec<&str> = vec![];

        text.push_str("\\begin{tabular}{@{}c@{}}\n    ");

        text.push_str(&self.structure.latex());
        text.push_str("\\\\");

        match self.structure {
            LatinStructure::Loop => {
                if self.commutative {
                    additional_rows.push("Commutative");
                }
            }
            LatinStructure::Quasigroup => {
                if self.commutative {
                    additional_rows.push("Commutative");
                } else if self.left_identity {
                    additional_rows.push("Left Identity");
                } else if self.right_identity {
                    additional_rows.push("Right Identity");
                }
            }
            _ => (),
        }

        for row in additional_rows {
            text.push_str("\\hline\n    ");
            text.push_str(row);
        }
        text.push_str("\\end{tabular}");

        text
    }
}

type AffineAutomorphism = (usize, usize, Sidedness);
pub type AllAffineAutomorphisms = (bool, Vec<AffineAutomorphism>);

#[derive(Clone)]
pub struct LatinSquareClassification {
    class: LatinType,
    index: usize,
    square: LatinSquare,
    all_permutations_all_affine_automorphisms: Vec<AllAffineAutomorphisms>,
}

impl LatinSquareClassification {
    pub fn fingerprint(&self) -> Bits {
        let mut fingerprint: Vec<bool> = match self.class.structure {
            LatinStructure::Quasigroup => vec![false, false],
            LatinStructure::Loop => vec![true, false],
            LatinStructure::Group => vec![false, true],
            LatinStructure::Abelian => vec![true, true],
        };

        for c in self
            .all_permutations_all_affine_automorphisms
            .iter()
        {
            fingerprint.push(c.0);
        }

        Bits {
            bits: fingerprint,
        }
    }
}

impl Display for LatinSquare {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut text: String = "".to_string();

        let mut first: bool = true;

        for row in self.0.iter() {
            if !first {
                text.push('\n');
            } else {
                first = false;
            }

            text.push_str(&row.first().unwrap().to_string());

            for v in row.iter().skip(1) {
                text.push(' ');
                text.push_str(&v.to_string());
            }
        }

        write!(f, "{}", &text)
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

pub fn classify_all_latin_squares(
    squares: &[LatinSquare],
    perms: &[Permutation],
) -> Vec<LatinSquareClassification> {
    let mut result: Vec<LatinSquareClassification> = vec![];

    for (j, s) in squares.iter().enumerate() {
        let mut all_affine_automorphisms: Vec<AllAffineAutomorphisms> =
            vec![(false, vec![]); perms.len()];

        for (i, p) in perms.iter().enumerate() {
            let mut w = s.clone();
            w.apply_permutation(p.clone());

            if w == *s {
                all_affine_automorphisms[i].0 = true;

                for v in 0..squares[0].0.len() {
                    for side in sidedness::SIDES {
                        let affine_automorphism = s.addition_permutation(v, &side).compose(p);
                        let found_permutation = perms
                            .iter()
                            .position(|x| x == &affine_automorphism)
                            .unwrap();
                        all_affine_automorphisms[found_permutation]
                            .1
                            .push((i, v, side));
                    }
                }
            }
        }

        result.push(LatinSquareClassification {
            class: s.classify_structure(),
            index: j,
            square: s.clone(),
            all_permutations_all_affine_automorphisms: all_affine_automorphisms,
        });
    }

    result
}

pub fn create_table(
    rows: Vec<LatinSquareClassification>,
) -> Table<SquareInformation, PermutationInformation> {
    let mut left: Vec<Vec<SquareInformation>> = vec![vec![
        SquareInformation::None,
        SquareInformation::None,
        SquareInformation::None,
    ]];
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
            right[i + 1].push(PermutationInformation::AllAffineAutomorphisms(
                affine_automorphisms.clone(),
            ));
        }
    }

    Table { left, right }
}
