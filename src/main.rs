#![allow(dead_code)]
#![allow(unused_variables)]

// Table is stored as the rows of the lower triangular representation of the group operation table.

use std::vec;

// Generates every triplet that has to be associative.
// (a, b, c) where a <= b <= c, but a != b != c
// TODO: Maybe optimize away the if statement somehow.
fn generate_associativity_triplets(n: usize) -> Vec<[usize; 3]> {
    let mut work: Vec<[usize; 3]> = vec![];
    for a in 0..(n-1) {
        for b in a..n {
            for c in b..n {
                if !(a == b && a == c) {
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
        table[*b][*a]
    } else {
        table[*a][*b]
    }
}

fn is_group_associative(table: &[Vec<usize>]) -> bool {
    let triplets = generate_associativity_triplets(table.len());

    for triplet in &triplets {
        let [a, b, c] = triplet;

        let l = group_add(table, a, b);
        let l = group_add(table, &l, c);

        let r = group_add(table, b, c);
        let r = group_add(table, a, &r);

        if l != r {
            return false
        }
    }

    true
}

// Assume table is already a valid table for the given values
// TODO: Test if table is empty (match statement in the beginning?)
fn group_generation_recursion(table: &Vec<Vec<usize>>, n: usize) -> Vec<Vec<Vec<usize>>> {
    
    // Check if finished table
    if table.last().unwrap().len() == n {
        if is_group_associative(table) {
            return vec![table.clone()]
        }
        return vec![]
    }

    let row: usize;
    let column: usize;

    // Finding the position of the next value
    if table[table.len() - 1].len() == table.len() {
        row = table.len();
        column = 0;
    } else {
        row = table.len() - 1;
        column = table[table.len() - 1].len();
    }
    
    // Find every value satisfying sudoku property and try again recursively.
    let mut result: Vec<Vec<Vec<usize>>> = vec![];
    for i in 0..n {
        if !table[row].contains(&i) && !table[column].contains(&i) {
            let mut working_table = table.clone();

            if column == 0 {
                working_table.push(vec![i]);
            } else {
                // TODO: Remove unwrap?
                working_table[table.len() - 1].push(i);
            }
            result.append(&mut group_generation_recursion(&working_table, n))
        }
    }

    result
}

fn generate_all_groups(n: usize) -> Vec<Vec<Vec<usize>>> {
    let work: Vec<Vec<Vec<usize>>> = vec![];

    group_generation_recursion(&vec![], n)
}

fn generate_all_sudocurity_groups(n: usize) -> Vec<Vec<Vec<usize>>> {
    todo!()
}

// TODO: Permutation struct
fn generate_all_permutations(n: usize) -> Vec<usize> {
    todo!()
}

fn main() {
    println!("Hello, world!");
    println!("{:?}", generate_associativity_triplets(4));
    println!("{:?}", generate_all_groups(3));
}
