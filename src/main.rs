#![allow(dead_code)]

use indicatif::ProgressBar;
use std::path::Path;
use std::{fs, time::Instant, vec};

// Table is stored as the rows of the upper triangular representation of the group operation table.
// Becase there was no benefit to storing as rows of lower triangular that could outweigh the benefit
// of easily reusing the same code for sudocurity tables.

// Generates every triplet that has to be associative.
// (a, b, c) where a <= b <= c, but a != b != c
fn generate_all_associativity_triplets(n: usize, zero_identity: bool) -> Vec<[usize; 3]> {
    let mut work: Vec<[usize; 3]> = vec![];
    let start = match zero_identity {
        true => 1,
        false => 0,
    };

    for a in start..(n - 1) {
        for b in a..n {
            for c in b..n {
                if a != c {
                    work.push([a, b, c])
                }
            }
        }
    }
    work
}

// Takes a and b and gives a + b where + is the group operation from the group operation table.
fn group_add(table: &[Vec<usize>], a: &usize, b: &usize) -> usize {
    if b > a {
        table[*a][b - a]
    } else {
        table[*b][a - b]
    }
}

fn is_group_associative(table: &[Vec<usize>]) -> bool {
    let triplets = match table[0][0] {
        0 => generate_all_associativity_triplets(table.len(), true),
        _ => generate_all_associativity_triplets(table.len(), false),
    };

    for triplet in &triplets {
        let [a, b, c] = triplet;

        let l = group_add(table, a, b);
        let l = group_add(table, &l, c);

        let r = group_add(table, b, c);
        let r = group_add(table, a, &r);

        if l != r {
            return false;
        }
    }

    true
}

// Assume table is already a valid and non-empty table for the given values
// TODO: Check associativity while running, not only at the end.
fn group_generation_recursion(table: &Vec<Vec<usize>>, n: usize) -> Vec<Vec<Vec<usize>>> {
    let mut result: Vec<Vec<Vec<usize>>> = vec![];

    if let Some(last_row) = table.last() {
        if table.len() == n {
            if is_group_associative(table) {
                return vec![table.clone()];
            }
            return vec![];
        }

        let row: usize;
        let column: usize;

        // Finding the position of the next value
        if last_row.len() == n - table.len() + 1 {
            row = table.len();
            column = table.len();
        } else {
            row = table.len() - 1;
            column = last_row.len() + table.len() - 1;
        }

        // Find every value satisfying sudoku property and try again recursively.
        if row == table.len() {
            'val: for i in 0..n {
                for (j, r) in table.iter().enumerate() {
                    if r[column - j] == i {
                        continue 'val;
                    }
                }
                let mut working_table = table.clone();
                working_table.push(vec![i]);
                result.append(&mut group_generation_recursion(&working_table, n))
            }
        } else {
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
                result.append(&mut group_generation_recursion(&working_table, n))
            }
        }
    }

    result
}

fn generate_all_groups(n: usize) -> Vec<Vec<Vec<usize>>> {
    let mut work: Vec<Vec<Vec<usize>>> = vec![];

    for i in 0..n {
        let working_table = vec![vec![i]];
        work.append(&mut group_generation_recursion(&working_table, n));
    }

    work
}

fn generate_all_sudocurity_groups(n: usize) -> Vec<Vec<Vec<usize>>> {
    let mut work: Vec<Vec<Vec<usize>>> = vec![];

    let v = (0..n).collect();

    let working_table = vec![v];
    work.append(&mut group_generation_recursion(&working_table, n));

    work
}

// Old functions above, new functions below. ###################################################################3

// TODO: Write cleaner with "?" operator.
// TODO: Can be optimized?
fn group_add_new(table: &[Vec<usize>], a: &usize, b: &usize) -> Option<usize> {
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

fn generate_all_groups_new(n: usize) -> Vec<Vec<Vec<usize>>> {
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

fn generate_all_sudocurity_groups_new(n: usize) -> Vec<Vec<Vec<usize>>> {
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

// TODO: Permutation struct
fn generate_all_permutations(n: usize) -> Vec<Vec<usize>> {
    permutation_recursion(n, vec![])
}

fn generate_sudocurity_permutations(n: usize) -> Vec<Vec<usize>> {
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
fn apply_permutation_to_group(table: &[Vec<usize>], permutation: &[usize]) -> Vec<Vec<usize>> {
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

fn print_pretty_table(table: &[Vec<usize>]) {
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

fn speedtest_group_generation(
    f: &dyn Fn(usize) -> Vec<Vec<Vec<usize>>>,
    n: usize,
) -> (Vec<u64>, Vec<usize>) {
    let mut timings: Vec<u64> = vec![];
    let mut sizes: Vec<usize> = vec![];

    for i in 1..=n {
        let time = Instant::now();
        let groups = f(i);
        let time = time.elapsed().as_secs();

        timings.push(time);
        sizes.push(groups.len());
    }

    (timings, sizes)
}

fn factorial(n: usize) -> usize {
    let mut result = 1;

    for i in 1..=n {
        result *= i
    }

    result
}

fn try_permutation_gives_automorphism(n: usize) {
    let length = factorial(n - 1);

    let bar = ProgressBar::new(TryInto::<u64>::try_into(length).unwrap() + 2_u64);

    let groups = generate_all_sudocurity_groups_new(n);
    bar.inc(1);
    let permutations = generate_sudocurity_permutations(n);
    bar.inc(1);

    let mut working_permutations: Vec<Vec<usize>> = vec![];
    let mut non_working_permutations: Vec<Vec<usize>> = vec![];

    'perm: for p in &permutations {
        bar.inc(1);
        for g in &groups {
            let test = apply_permutation_to_group(g, p);
            // assert!(groups.contains(&test));

            if test == *g {
                working_permutations.push(p.clone());
                continue 'perm;
            }

            // println!("---------");
            // print_pretty_table(&test);
            // print_pretty_table(g);
        }
        non_working_permutations.push(p.clone());
    }

    println!("Working:");
    for w in working_permutations.iter().take(10) {
        println!("{:?}", w);
    }
    println!("Non-working:");
    for w in non_working_permutations.iter().take(10) {
        println!("{:?}", w)
    }
    println!(
        "{}, {}",
        working_permutations.len(),
        non_working_permutations.len()
    );
}

fn try_permutation_is_group_op(n: usize) {
    let length = factorial(n - 1);

    let bar = ProgressBar::new(TryInto::<u64>::try_into(length).unwrap() + 2_u64);

    let groups = generate_all_groups_new(n);
    bar.inc(1);
    let permutations = generate_all_permutations(n);
    bar.inc(1);

    let mut working_permutations: Vec<Vec<usize>> = vec![];
    let mut non_working_permutations: Vec<Vec<usize>> = vec![];

    'perm: for p in &permutations {
        bar.inc(1);
        for g in &groups {
            if g.contains(p) {
                working_permutations.push(p.clone());
                continue 'perm;
            }
        }
        non_working_permutations.push(p.clone());
    }

    println!("Working:");
    for w in working_permutations.iter().take(10) {
        println!("{:?}", w);
    }
    println!("Non-working:");
    for w in non_working_permutations.iter().take(10) {
        println!("{:?}", w)
    }
    println!(
        "{}, {}",
        working_permutations.len(),
        non_working_permutations.len()
    );
}

fn try_exist_perm_for_every_group_gives_automorphism(n: usize) {
    let groups = generate_all_sudocurity_groups_new(n);

    let length = groups.len();

    let bar = ProgressBar::new(TryInto::<u64>::try_into(length).unwrap() + 2_u64);

    bar.inc(1);
    let permutations = generate_sudocurity_permutations(n);
    bar.inc(1);

    let mut working_groups: Vec<Vec<Vec<usize>>> = vec![];
    let mut non_working_groups: Vec<Vec<Vec<usize>>> = vec![];

    'groups: for g in &groups {
        bar.inc(1);
        for p in permutations.iter().skip(1) {
            let t = apply_permutation_to_group(g, p);
            if &t == g {
                working_groups.push(g.clone());
                continue 'groups;
            }
        }
        non_working_groups.push(g.clone());
    }

    println!("Working:");
    for w in &working_groups {
        print_pretty_table(w);
    }
    println!("Non-working:");
    for w in &non_working_groups {
        print_pretty_table(w);
    }
    println!(
        "Working: {}, Non-working: {}",
        working_groups.len(),
        non_working_groups.len()
    );
}

fn compose_affine_permutation(
    permutation: &[usize],
    table: &[Vec<usize>],
    row: &usize,
) -> Vec<usize> {
    let mut perm: Vec<usize> = vec![];

    for p in permutation.iter() {
        let a = group_add_new(table, p, row).unwrap();

        perm.push(a);
    }

    perm
}

fn try_permutation_affine_automorphism(n: usize) {
    let length = factorial(n);

    let bar = ProgressBar::new(TryInto::<u64>::try_into(length).unwrap() + 2_u64);

    let groups = generate_all_groups_new(n);
    bar.inc(1);
    let permutations = generate_all_permutations(n);
    bar.inc(1);

    let mut working_permutations: Vec<Vec<usize>> = vec![];
    let mut non_working_permutations: Vec<Vec<usize>> = vec![];

    'perm: for p in &permutations {
        bar.inc(1);
        for g in &groups {
            for i in 0..g.len() {
                let perm = compose_affine_permutation(p, g, &i);

                let t = apply_permutation_to_group(g, &perm);

                if g == &t {
                    working_permutations.push(p.clone());

                    continue 'perm;
                }
            }
        }
        non_working_permutations.push(p.clone());
    }

    println!("Working:");
    for w in working_permutations.iter().take(100) {
        println!("{:?}", w);
    }
    println!("Non-working:");
    for w in non_working_permutations.iter().take(100) {
        println!("{:?}", w)
    }
    println!(
        "{}, {}",
        working_permutations.len(),
        non_working_permutations.len()
    );
}

fn main() {
    println!("Hello, world!");

    try_permutation_affine_automorphism(5);
}

#[cfg(test)]
mod tests {
    use super::*;

    // fn test_generate_all_associativity_triplets() {
    //     todo!()
    // }

    // fn test_group_add() {
    //     todo!()
    // }

    // fn test_is_group_associative() {
    //     todo!()
    // }

    // fn test_group_generation_recursion() {
    //     todo!()
    // }

    // fn test_generate_all_groups() {
    //     todo!()
    // }

    // fn test_generate_all_sudocurity_groups() {
    //     todo!()
    // }

    #[test]
    fn test_compare_all_group_generation_old_and_new() {
        let n = 6;

        let groups_old = generate_all_groups(n);
        let groups_new = generate_all_groups_new(n);

        assert_eq!(groups_old, groups_new);
    }

    #[test]
    fn test_compare_sudocurity_group_generation_old_and_new() {
        let n = 6;

        let groups_old = generate_all_sudocurity_groups(n);
        let groups_new = generate_all_sudocurity_groups_new(n);

        assert_eq!(groups_old, groups_new);
    }

    #[test]
    fn test_apply_permutation_to_group() {
        let table = generate_all_sudocurity_groups_new(5);

        let permutation = vec![0, 2, 3, 4, 1];

        let new_t = apply_permutation_to_group(&table[3], &permutation);

        assert_eq!(new_t, table[2]);
    }

    #[test]
    fn test_apply_permutation_to_group_2() {
        let n = 7;

        let groups = generate_all_sudocurity_groups_new(n);
        let permutations = generate_sudocurity_permutations(n);

        for p in permutations.iter() {
            for g in &groups {
                let test = apply_permutation_to_group(g, p);

                assert!(groups.contains(&test))
            }
        }
    }
}
