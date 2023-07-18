use std::fmt::Display;

// Permutation on the form: x -> p[x] is a permutation.
#[derive(Debug, Clone, PartialEq)]
pub struct Permutation(pub Vec<usize>);

fn permutation_recursion(n: usize, part_of_permutation: Vec<usize>) -> Vec<Vec<usize>> {
    if part_of_permutation.len() == n {
        return vec![part_of_permutation];
    }

    let mut result: Vec<Vec<usize>> = vec![];

    for i in 0..n {
        if !part_of_permutation.contains(&i) {
            let mut new_permutation = part_of_permutation.clone();
            new_permutation.push(i);
            result.append(&mut permutation_recursion(n, new_permutation));
        }
    }

    result
}

pub fn generate_all_permutations(n: usize) -> Vec<Vec<usize>> {
    permutation_recursion(n, vec![])
}

impl Permutation {
    // Generates all permutations.
    pub fn generate_all(n: usize) -> Vec<Permutation> {
        let mut perms = generate_all_permutations(n);
        let mut result: Vec<Permutation> = vec![];
        for p in perms.drain(..) {
            result.push(Permutation(p));
        }
        result
    }

    pub fn compose(&self, inner: &Permutation) -> Permutation {
        let mut result: Vec<usize> = vec![];

        for i in 0..self.0.len() {
            result.push(self.0[inner.0[i]]);
        }

        Permutation(result)
    }

    pub fn print(&self) {
        todo!()
    }
}

impl Display for Permutation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut text: String = "".to_string();

        text.push('0');

        for p in 1..self.0.len() {
            text.push(' ');
            text.push_str(&p.to_string());
        }
        text.push('\n');

        text.push_str(&self.0[0].to_string());

        for p in self.0.iter().skip(1) {
            text.push(' ');
            text.push_str(&p.to_string());
        }

        write!(f, "{}", text)
    }
}
