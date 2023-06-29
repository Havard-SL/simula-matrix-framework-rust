#![allow(dead_code)]
#![allow(unused_variables)]

use std::time::Instant;

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

    for a in start..(n-1) {
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
            return false
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
                return vec![table.clone()]
            }
            return vec![]
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

fn print_pretty_table(table: &[Vec<usize>]) {
    let n = table.len()*3 + 3;
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

fn generate_all_sudocurity_groups(n: usize) -> Vec<Vec<Vec<usize>>> {
    let mut work: Vec<Vec<Vec<usize>>> = vec![];

    let v = (0..n).collect();

    let working_table = vec![v];
    work.append(&mut group_generation_recursion(&working_table, n));

    work
}

// TODO: Permutation struct
fn generate_all_permutations(n: usize) -> Vec<usize> {
    todo!()
}

fn main() {
    println!("Hello, world!");
    // println!("{:?}", generate_associativity_triplets(4));
    let time = Instant::now();
    let groups = generate_all_sudocurity_groups(8);
    let time = time.elapsed().as_secs();
    // for g in &groups {
    //     print_pretty_table(g);
    // }
    println!("{}", groups.len());
    println!("{}", time);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_generate_all_associativity_triplets() {
        todo!()
    }

    fn test_group_add() {
        todo!()
    }

    fn test_is_group_associative() {
        todo!()
    }

    fn test_group_generation_recursion() {
        todo!()
    }

    fn test_generate_all_groups() {
        todo!()
    }

    fn test_generate_all_sudocurity_groups() {
        todo!()
    }
}