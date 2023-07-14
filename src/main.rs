#![allow(dead_code)]

mod common;

mod latin_square;

use latin_square::sidedness;
use latin_square::LatinSquare;
use latin_square::LatinStructure;
use latin_square::Permutation;
use latin_square::Sidedness;

mod abelian;

mod experiments;

mod affine_automorphism_table;

mod table;

mod traits;
use traits::LaTeX;

use latin_square::classify_all_latin_squares;
use latin_square::create_table;

// TODO: Methods vs standalone functions.
fn main() {
    // Set the dimension of the Latin squares i generate.
    let n = 3;

    // Generate all the n by n latin squares.
    let squares = LatinSquare::generate_all(n);

    // Generate all the permutations on n elements.
    let perms = Permutation::generate_all(n);

    let mut classification = classify_all_latin_squares(&squares, &perms);

    classification.sort_by_cached_key(|x| x.fingerprint());

    let table = create_table(classification);

    let text = table.latex();

    println!("{text}");
}
