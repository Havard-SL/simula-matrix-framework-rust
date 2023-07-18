use indicatif::ProgressBar;
use std::time::Instant;

use crate::abelian::*;
use crate::common::factorial;
use crate::latin_square::permutation::generate_all_permutations;

pub fn speedtest_group_generation(
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

pub fn try_permutation_gives_automorphism(n: usize) {
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

pub fn try_permutation_is_group_op(n: usize) {
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

pub fn try_exist_perm_for_every_group_gives_automorphism(n: usize) {
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

pub fn compose_affine_permutation(
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

pub fn try_permutation_affine_automorphism(n: usize) {
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

pub fn try_permutations_equal_in_isomorphism_class(n: usize) {
    let length = factorial(n - 1);

    let bar = ProgressBar::new(TryInto::<u64>::try_into(length).unwrap() + 2_u64);

    let groups = generate_all_sudocurity_groups_new(n);
    bar.inc(1);
    let permutations = generate_sudocurity_permutations(n);
    bar.inc(1);

    let mut working_permutations: Vec<Vec<Vec<usize>>> = vec![];

    for g in &groups {
        bar.inc(1);
        let mut working: Vec<Vec<usize>> = vec![];

        for p in &permutations {
            let test = apply_permutation_to_group(g, p);

            if &test == g {
                working.push(p.clone())
            }
        }
        working_permutations.push(working);
    }

    for p in working_permutations.iter() {
        println!("{:?}", p);
    }

    println!("{}", working_permutations.len(),);
}
