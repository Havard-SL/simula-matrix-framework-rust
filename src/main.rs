#![allow(dead_code)]

mod obsolete;

mod structs;
use structs::*;
// use traits::LaTeX;

mod common;

mod spreadsheet;

// use table::create_complete_table;
// use spreadsheet::write_table_to_spreadsheet;

// TODO: Methods vs standalone functions.
fn main() {
    // Set the dimension of the Latin squares i generate.
    let n = 5;

    // Generate all the n by n latin squares.
    let squares = LatinSquare::generate_all(n); // [..1000].to_vec();

    // Generate all the permutations on n elements.
    let perms = Permutation::generate_all(n);

    let classification: Vec<LatinSquareClassification> =
        structs::latin_square::classify_all_latin_squares(&squares, &perms);

    let table = table::create_summary_table(classification, &perms);

    spreadsheet::write_table_to_spreadsheet(&table, n, "summary").unwrap();
}
