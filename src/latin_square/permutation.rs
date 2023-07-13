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