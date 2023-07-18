#![allow(dead_code)]

mod structs;
use structs::*;

mod common;

mod spreadsheet;

// TODO: Methods vs standalone functions.
fn main() {
    // Set the dimension of the Latin squares i generate.
    let n = 5;

    // let v_1 = Bits { bits: vec![true, true, false, true, false, true, true] };
    // let v_2 = Bits { bits: vec![false, true, false, true, false, true, true] };

    // println!("{:?}", v_2.cmp(&v_2));

    // Generate all the n by n latin squares.
    let squares = LatinSquare::generate_all(n); // [..1000].to_vec();

    // Generate all the permutations on n elements.
    let perms = Permutation::generate_all(n);

    let mut classification: Vec<LatinSquareClassification> =
        structs::latin_square::classify_all_latin_squares(&squares, &perms);

    classification.sort_by_cached_key(|x| x.fingerprint());

    let table = structs::table::create_table(classification);

    spreadsheet::write_table_to_spreadsheet(&table).unwrap();

    // let text = table.latex();

    // println!("{text}");
}
