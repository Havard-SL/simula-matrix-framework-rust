use super::LatinSquareClassification;
use super::Permutation;
use super::SquareInformation;

// A table, where left[i] + right[i] becomes one row in the table.
// Split it up in order to make the abstraction cleaner and maybe slightly faster?
pub struct Table<T> {
    pub table: Vec<Vec<T>>,
}

pub fn create_complete_table(
    mut rows: Vec<LatinSquareClassification>,
    perms: &[Permutation],
) -> Table<SquareInformation> {
    rows.sort_by_cached_key(|x| x.fingerprint());

    let mut table: Vec<Vec<SquareInformation>> = vec![
        vec![
            SquareInformation::None,
            SquareInformation::None,
            SquareInformation::None,
        ],
        vec![
            SquareInformation::None,
            SquareInformation::None,
            SquareInformation::None,
        ],
    ];

    for (i, p) in perms.iter().enumerate() {
        table[0].push(SquareInformation::Permutation(p.clone()));
        table[1].push(SquareInformation::PermutationIndex(i));
    }

    for (i, s) in rows.iter().enumerate() {
        table.push(vec![]);

        table[i + 2].push(SquareInformation::LatinSquare(s.square.clone()));
        table[i + 2].push(SquareInformation::LatinSquareIndex(s.index));
        table[i + 2].push(SquareInformation::Class(s.class.clone()));

        for affine_automorphisms in s.all_permutations_all_affine_automorphisms.iter() {
            table[i + 2].push(SquareInformation::AllAffineAutomorphisms(
                affine_automorphisms.clone(),
            ));
        }
    }

    Table { table }
}

pub fn create_summary_table(
    mut rows: Vec<LatinSquareClassification>,
    perms: &[Permutation],
) -> Table<SquareInformation> {
    rows.sort_by_cached_key(|x| x.fingerprint_no_structure());

    let mut previous_fingerprint = rows.first().unwrap().fingerprint_no_structure();

    let mut split_indices: Vec<usize> = vec![];

    let mut split_point: usize = 0;

    for (i, r) in rows.iter_mut().enumerate() {
        let f = r.fingerprint_no_structure();

        if f > previous_fingerprint {
            split_indices.push(i - split_point);
            previous_fingerprint = f;
            split_point = i;
        }
    }

    split_indices.push(rows.len() - split_point);

    let mut table: Vec<Vec<SquareInformation>> =
        vec![vec![SquareInformation::None], vec![SquareInformation::None]];
    let mut sum_information: Vec<Vec<(usize, usize)>> = vec![];

    for (i, p) in perms.iter().enumerate() {
        table[0].push(SquareInformation::Permutation(p.clone()));
        table[1].push(SquareInformation::PermutationIndex(i));
    }

    let mut second: &mut [LatinSquareClassification] = &mut rows;

    for (f, i) in split_indices.into_iter().enumerate() {
        let (s_1, s_2) = second.split_at_mut(i);

        second = s_2;

        let mut sum: Vec<(usize, usize)> = vec![(0, 0); perms.len()];

        for s in s_1 {
            for (j, c) in s
                .all_permutations_all_affine_automorphisms
                .iter()
                .enumerate()
            {
                if c.0 {
                    sum[j].0 += 1;
                }
                if !c.1.is_empty() {
                    sum[j].1 += 1;
                }
            }
        }

        table.push(vec![SquareInformation::FingerprintIndex(f)]);

        for aut_aff in &sum {
            table[f + 2].push(SquareInformation::AutomorphismAndAffineSums(*aut_aff));
        }

        sum_information.push(sum);
    }

    table.push(vec![SquareInformation::Text("Sum W/o 0".to_string())]);
    table.push(vec![SquareInformation::Text("Sum All".to_string())]);

    let mut sum: Vec<(usize, usize)> = vec![];

    for (i, _) in sum_information[0].iter().enumerate() {
        let mut working_sum: (usize, usize) = (0, 0);

        for r in sum_information.iter().skip(1) {
            working_sum.0 += r[i].0;
            working_sum.1 += r[i].1;
        }

        sum.push(working_sum)
    }
    let length = table.len();

    for c in &sum {
        table[length - 2].push(SquareInformation::AutomorphismAndAffineSums(*c));
    }

    for (i, c) in sum.iter_mut().enumerate() {
        c.0 += sum_information[0][i].0;
        c.1 += sum_information[0][i].1;
    }

    for c in &sum {
        table[length - 1].push(SquareInformation::AutomorphismAndAffineSums(*c));
    }

    Table { table }
}
