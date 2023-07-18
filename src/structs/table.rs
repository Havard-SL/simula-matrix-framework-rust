use super::LatinSquareClassification;
use super::PermutationInformation;
use super::SquareInformation;

// A table, where left[i] + right[i] becomes one row in the table.
// Split it up in order to make the abstraction cleaner and maybe slightly faster?
pub struct Table<L, R> {
    pub left: Vec<Vec<L>>,
    pub right: Vec<Vec<R>>,
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
