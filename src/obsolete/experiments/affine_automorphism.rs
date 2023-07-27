use crate::structs::LatinSquare;
use crate::structs::Permutation;

// Check and print if there are any permutations that are not an automorphism for any latin square.
// Check and print if there are any latin squares that only has the identity as an automorphism.
pub fn try_automorphism_groups_porperties(
    automorphisms_given_group: Vec<Vec<usize>>,
    squares: Vec<LatinSquare>,
    perms: Vec<Permutation>,
) {
    let mut j: usize = 0;

    'i: for (i, p) in perms.iter().enumerate() {
        for row in &automorphisms_given_group {
            if row.binary_search(&i).is_ok() {
                continue 'i;
            }
        }

        j += 1;

        println!("Permutation number {i}, {:?} is no automorphism.", p);

        if j == 10 {
            println!("Too many permutations, stopped.");
            break;
        }
    }

    let mut j: usize = 0;

    for (i, row) in automorphisms_given_group.iter().enumerate() {
        if row == &vec![0] {
            println!("Square number {i}: ");
            squares[i].print();
            println!("Has only trivial automorphisms.");

            j += 1;

            if j == 10 {
                println!("Too many squares, stopped.");
                break;
            }
        }
    }
}