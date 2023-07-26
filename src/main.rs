#![allow(dead_code)]

mod obsolete;

mod structs;
use structs::*;
use traits::LaTeX;

mod common;

mod spreadsheet;

fn create_complete_spreadsheet(n: usize) {
    // Generate all the n by n latin squares.
    let squares = LatinSquare::generate_all(n); // [..1000].to_vec();

    // Generate all the permutations on n elements.
    let perms = Permutation::generate_all(n);

    // Calculate the classifications (AAuts, Auts, Class, etc... for every latin square)
    let classification: Vec<LatinSquareClassification> =
        structs::latin_square::classify_all_latin_squares(&squares, &perms);

    // Turn the classifications into the table format that can be exported.
    let table: Table<SquareInformation> = table::create_complete_table(classification, &perms);

    // Export table as a spreadsheet,
    spreadsheet::write_table_to_spreadsheet(&table, n, "complete").unwrap();
}

fn create_summary_spreadsheet(n: usize) {
    // Generate all the n by n latin squares.
    let squares = LatinSquare::generate_all(n); // [..1000].to_vec();

    // Generate all the permutations on n elements.
    let perms = Permutation::generate_all(n);

    // Calculate the classifications (AAuts, Auts, Class, etc... for every latin square)
    let classification: Vec<LatinSquareClassification> =
        structs::latin_square::classify_all_latin_squares(&squares, &perms);

    // Turn the classifications into the table format that can be exported.
    let table = table::create_summary_table(classification, &perms);

    // Export table as a spreadsheet,
    spreadsheet::write_table_to_spreadsheet(&table, n, "summary").unwrap();
}

fn create_latex_table(n: usize) {
    // Generate all the n by n latin squares.
    let squares = LatinSquare::generate_all(n); // [..1000].to_vec();

    // Generate all the permutations on n elements.
    let perms = Permutation::generate_all(n);

    // Calculate the classifications (AAuts, Auts, Class, etc... for every latin square)
    let classification: Vec<LatinSquareClassification> =
        structs::latin_square::classify_all_latin_squares(&squares, &perms);

    // Turn the classifications into the table format that can be exported.
    let table = table::create_summary_table(classification, &perms);
    // let table: Table<SquareInformation> = table::create_complete_table(classification, &perms);

    println!("{}", table.latex())
}

fn create_ascii_table(n: usize) {
    // Generate all the n by n latin squares.
    let squares = LatinSquare::generate_all(n); // [..1000].to_vec();

    // Generate all the permutations on n elements.
    let perms = Permutation::generate_all(n);

    // Print the ASCII table
    obsolete::affine_automorphism_table::print_affine_automorphism_table(&squares, &perms);
}

// TODO: Methods vs standalone functions.
fn main() {

    // Create the summary spreadsheet for 5x5 latin squares.
    // create_summary_spreadsheet(5);

    // Create the complete spreadsheet for 4x4 latin squares.
    // create_complete_spreadsheet(4);

    // Print the latex table for 4x4 latin squares (comment and uncomment type of table needed in the function above).
    // create_latex_table(4);

    // Creates the ASCII table for 3x3 latin squares.
    // create_ascii_table(3);

    // #############################

    // Run an experiment.
    // obsolete::experiments::{abelian, latin_square}::{name of experiment}
    // Example:
    // obsolete::experiments::abelian::try_permutation_gives_automorphism(5);

    // #############################

    // Generate every abelian group of order 6.
    // let every_abelian_group = obsolete::abelian::generate_all_groups_new(6);

    // // Create an (old, non struct) permutation that flips 4 and 5.
    // let flipping_permutation = vec![0, 1, 2, 3, 5, 4];

    // // Create the conjugate/conjugacy of a the 23rd abelian group with respect to flipping_permutation.
    // let conjugate = obsolete::abelian::apply_permutation_to_group(&every_abelian_group[22], &flipping_permutation);

    // // Print the group operation table of conjugate in ASCII format.
    // obsolete::abelian::print_pretty_table(&conjugate);

    // // Generate every abelian group with fixed identity 0 of order 9.
    // let every_fixed_identity_abelian_group = obsolete::abelian::generate_all_sudocurity_groups_new(9);

    // // Generate every permutation with fixed identity of order 9.
    // let every_fixed_0_permutation = obsolete::abelian::generate_sudocurity_permutations(9);
}
