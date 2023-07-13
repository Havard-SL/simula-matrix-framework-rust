use indicatif::ProgressBar;

pub mod permutation;
pub use permutation::Permutation;

pub use self::sidedness::Sidedness;

pub mod sidedness;

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

    // Classifies the Latin square as a quasigroup, loop, group or abelian group.
    pub fn classify(&self) -> LatinStructure {
        // Check if it contains a right-identity
        let mut standard: Vec<usize> = (0..self.0.len()).collect();

        if !self.0.contains(&standard) {
            return LatinStructure::Quasigroup;
        }

        // Check if it contains a left-identity
        let column = self.0[0].iter().position(|&x| x == 0).unwrap();

        for row in self.0.iter().rev() {
            let c = standard.pop().unwrap();

            if c != row[column] {
                return LatinStructure::Quasigroup;
            }
        }

        // Check if associative
        let a: Vec<usize>;

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
        for a in 1..self.0.len() {
            for b in 0..a {
                if self.0[a][b] != self.0[b][a] {
                    return LatinStructure::Group;
                }
            }
        }

        LatinStructure::Abelian
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