use super::LaTeX;

use super::AffineAutomorphism;
use super::AllAffineAutomorphisms;

use super::Sidedness;

pub fn permutation_recursion(n: usize, part_of_permutation: Vec<usize>) -> Vec<Vec<usize>> {
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

// Permutation on the form: x -> p[x] is a permutation.
#[derive(Debug, Clone, PartialEq)]
pub struct Permutation(pub Vec<usize>);

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
        let mut text: String = "".to_string();

        let b = !self.1.is_empty();

        match (self.0, b) {
            (false, false) => (),
            (true, false) => text.push_str("\\cellcolor{blue}"),
            (false, true) => text.push_str("\\cellcolor{yellow}"),
            (true, true) => text.push_str("\\cellcolor{green}"),
        }

        text.push_str("\\begin{tabular}{@{}c@{}}\n    ");

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
