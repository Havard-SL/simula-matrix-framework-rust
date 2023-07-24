use indicatif::ProgressBar;
use std::fs;
use std::path::Path;

pub mod legacy_abelian;
use legacy_abelian::generate_all_associativity_triplets;
use crate::structs::permutation::permutation_recursion;

// TODO: Write cleaner with "?" operator.
// TODO: Can be optimized?
pub fn group_add_new(table: &[Vec<usize>], a: &usize, b: &usize) -> Option<usize> {
    if b > a {
        if let Some(row) = table.get(*a) {
            // if let Some(column) = row.get(b - a) {
            //     Some(*column)
            // } else {
            //     None
            // }
            row.get(b - a).copied()
        } else {
            None
        }
    } else if let Some(row) = table.get(*b) {
        // if let Some(column) = row.get(a - b) {
        //     Some(*column)
        // } else {
        //     None
        // }
        row.get(a - b).copied()
    } else {
        None
    }
}

// TODO: Write cleaner with "?" operator.
// TODO: Can be optimized?
fn test_triplet(table: &[Vec<usize>], triplet: &[usize; 3]) -> Option<bool> {
    let [a, b, c] = triplet;

    let l = if let Some(l) = group_add_new(table, a, b) {
        // if let Some(l) = group_add_new(table, &l, c) {
        //     l
        // } else {
        //     return None
        // }
        group_add_new(table, &l, c)?
    } else {
        return None;
    };

    let r = if let Some(r) = group_add_new(table, b, c) {
        // if let Some(r) = group_add_new(table, a, &r) {
        //     r
        // } else {
        //     return None
        // }
        group_add_new(table, a, &r)?
    } else {
        return None;
    };

    if l != r {
        return Some(false);
    }
    Some(true)
}

// Takes a table, and tests associativity.
// Returns None if it is not associative. Returns Some(v) with a vector of the remaining associativity checks.
// TODO: Can be optimized?
fn check_associativity(
    table: &[Vec<usize>],
    remaining_associatvity_checks: &[[usize; 3]],
) -> Option<Vec<[usize; 3]>> {
    let mut new_associativity_checks: Vec<[usize; 3]> = vec![];

    for triplet in remaining_associatvity_checks.iter() {
        match test_triplet(table, triplet) {
            None => new_associativity_checks.push(*triplet),
            Some(b) if !b => return None,
            Some(_) => continue,
        }
        // if let Some(b) = test_triplet(table, triplet) {
        //     if !b {
        //         return (false, vec![]);
        //     }
        // } else {
        //     new_associativity_checks.push(*triplet);
        // }
    }

    Some(new_associativity_checks)
}

// Takes a table and tests the remaining associativity checks.
// Returns None if not associative. Returns Some(v) with a subslice of the remaining_associativity_checks.
fn check_associativity_advanced<'a>(
    table: &[Vec<usize>],
    remaining_associatvity_checks: &'a [[usize; 3]],
) -> Option<&'a [[usize; 3]]> {
    let mut remove_checks_indices: Vec<usize> = vec![];

    for (i, triplet) in remaining_associatvity_checks.iter().enumerate() {
        match test_triplet(table, triplet) {
            None => continue,
            Some(b) if !b => return None,
            Some(_) => remove_checks_indices.push(i),
        }
        // if let Some(b) = test_triplet(table, triplet) {
        //     if !b {
        //         return (false, vec![]);
        //     }
        // } else {
        //     new_associativity_checks.push(*triplet);
        // }
    }

    // for i in kept_checks_indices.iter().rev() {
    //     let (a, b) = remaining_associatvity_checks.split(pred)
    // }

    // Some(new_associativity_checks);

    todo!()
}

// TODO: Overlapping code at the bottom
fn group_generation_recursion_new(
    table: &Vec<Vec<usize>>,
    n: usize,
    remaining_associatvity_checks: &[[usize; 3]],
) -> Vec<Vec<Vec<usize>>> {
    let mut result: Vec<Vec<Vec<usize>>> = vec![];

    if let Some(last_row) = table.last() {
        if table.len() == n {
            return vec![table.to_vec()];
        }

        let row: usize;
        let column: usize;

        // Finding the position of the next value
        // Find every value satisfying sudoku property and associativity and try again recursively.

        if last_row.len() == n - table.len() + 1 {
            column = table.len();

            'val: for i in 0..n {
                for (j, r) in table.iter().enumerate() {
                    if r[column - j] == i {
                        continue 'val;
                    }
                }
                let mut working_table = table.to_vec();
                working_table.push(vec![i]);

                if let Some(remaining_checks) =
                    check_associativity(&working_table, remaining_associatvity_checks)
                {
                    result.append(&mut group_generation_recursion_new(
                        &working_table,
                        n,
                        &remaining_checks,
                    ))
                }
            }
        } else {
            row = table.len() - 1;
            column = last_row.len() + table.len() - 1;

            'val: for i in 0..n {
                if table[row].contains(&i) {
                    continue 'val;
                }
                for (j, r) in table.iter().take(row).enumerate() {
                    if r[row - j] == i || r[column - j] == i {
                        continue 'val;
                    }
                }
                let mut working_table = table.clone();
                working_table.last_mut().unwrap().push(i);

                if let Some(remaining_checks) =
                    check_associativity(&working_table, remaining_associatvity_checks)
                {
                    result.append(&mut group_generation_recursion_new(
                        &working_table,
                        n,
                        &remaining_checks,
                    ))
                }
            }
        }
    }

    result
}

fn generate_path(n: usize, sudocurity: bool) -> String {
    let mut path = "data/".to_string();

    if sudocurity {
        path.push_str("sudocurity/")
    } else {
        path.push_str("all/")
    }

    path.push_str(&n.to_string());

    path.push_str(".json");

    path
}

fn load_groups(n: usize, sudocurity: bool) -> Option<Vec<Vec<Vec<usize>>>> {
    let path = generate_path(n, sudocurity);
    let path = Path::new(&path);

    if !path.exists() {
        return None;
    }

    let groups = fs::read_to_string(path).unwrap();
    let groups: Vec<Vec<Vec<usize>>> = serde_json::from_str(&groups).unwrap();

    Some(groups)
}

fn save_groups(n: usize, sudocurity: bool, groups: &Vec<Vec<Vec<usize>>>) {
    let path = generate_path(n, sudocurity);
    let path = Path::new(&path);

    let groups = serde_json::to_string(groups).unwrap();
    fs::write(path, groups).unwrap()
}

pub fn generate_all_groups_new(n: usize) -> Vec<Vec<Vec<usize>>> {
    if let Some(groups) = load_groups(n, false) {
        return groups;
    }

    let mut work: Vec<Vec<Vec<usize>>> = vec![];

    let triplets = generate_all_associativity_triplets(n, false);

    let bar = ProgressBar::new(n as u64);

    for i in 0..n {
        let working_table = vec![vec![i]];

        work.append(&mut group_generation_recursion_new(
            &working_table,
            n,
            &triplets,
        ));
        bar.inc(1);
    }

    save_groups(n, false, &work);

    work
}

pub fn generate_all_sudocurity_groups_new(n: usize) -> Vec<Vec<Vec<usize>>> {
    if let Some(groups) = load_groups(n, true) {
        return groups;
    }

    let mut work: Vec<Vec<Vec<usize>>> = vec![];

    if n == 1 {
        work = vec![vec![vec![0]]];
    } else {
        let bar = ProgressBar::new(n as u64 - 1);

        let triplets = generate_all_associativity_triplets(n, true);

        let v = (0..n).collect();

        let working_table = vec![v];

        let mut v = vec![0];
        v.append(&mut (2..n).collect());

        for i in v {
            let mut w = working_table.clone();

            w.push(vec![i]);

            work.append(&mut group_generation_recursion_new(&w, n, &triplets));
            bar.inc(1);
        }
    }

    save_groups(n, true, &work);

    work
}

pub fn generate_sudocurity_permutations(n: usize) -> Vec<Vec<usize>> {
    permutation_recursion(n, vec![0])
}

// a < b
fn swap_rows_and_columns(table: &mut [Vec<usize>], a: usize, b: usize) {
    let mut a = a;
    let mut b = b;

    if a > b {
        (a, b) = (b, a);
    }

    let length = table.len();

    let (first, last) = table.split_at_mut(b);

    // Part 1
    for (i, row) in first.iter_mut().take(a).enumerate() {
        row.swap(a - i, b - i)
    }

    // Part 2
    // Need to check index?
    if b != length {
        first[a][(b - a + 1)..].swap_with_slice(&mut last[0][1..]);
    }

    // Part 3
    first[a][0..1].swap_with_slice(&mut last[0][0..1]);

    // Part 4
    if b != a + 1 {
        let (first_1, first_2) = first.split_at_mut(a + 1);

        for (i, f) in first_2.iter_mut().enumerate().take(b - a - 1) {
            let s_1 = &mut first_1[a][(i + 1)..(i + 2)];
            let s_2 = &mut f[(b - a - i - 1)..(b - a - i)];
            s_1.swap_with_slice(s_2);
        }
    }
}

// Assume length of table is equal to length of permutation.
pub fn apply_permutation_to_group(table: &[Vec<usize>], permutation: &[usize]) -> Vec<Vec<usize>> {
    let mut working_table = table.to_vec();

    for row in &mut working_table {
        for column in row {
            *column = permutation[*column];
        }
    }

    let mut working_permutation = permutation.to_vec();
    for i in 0..table.len() {
        let j = working_permutation.iter().position(|x| x == &i).unwrap();

        if i != j {
            swap_rows_and_columns(&mut working_table, i, j);

            working_permutation.swap(i, j);
        }
    }

    working_table
}

pub fn print_pretty_table(table: &[Vec<usize>]) {
    let n = table.len() * 3 + 3;
    let border = "-".repeat(n);
    println!("{border}");
    for row in 0..table.len() {
        let mut string = "|".to_string();
        for column in 0..table.len() {
            let val = match (row, column) {
                (row, column) if row > column => table[column][row - column],
                (_, _) => table[row][column - row],
            };
            if val < 10 {
                string.push_str("  ");
                string.push_str(&val.to_string());
            } else {
                string.push(' ');
                string.push_str(&val.to_string());
            }
        }
        string.push_str(" |");
        println!("{string}");
    }
    println!("{border}");
}
