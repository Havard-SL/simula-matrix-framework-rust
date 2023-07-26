use indicatif::ProgressBar;

use crate::common::factorial;
use crate::latin_square::*;

use crate::structs::LatinStructure;
use crate::structs::Permutation;

// Checks if the class is exactly preserved under conjugacy.
pub fn try_class_preserved_after_conjugacy(n: usize) {
    let squares = LatinSquare::generate_all(n);

    let mut classifications: (
        Vec<LatinSquare>,
        Vec<LatinSquare>,
        Vec<LatinSquare>,
        Vec<LatinSquare>,
    ) = (vec![], vec![], vec![], vec![]);

    for s in &squares {
        let c = s.classify();

        match c {
            LatinStructure::Quasigroup => classifications.0.push(s.clone()),
            LatinStructure::Loop => classifications.1.push(s.clone()),
            LatinStructure::Group => classifications.2.push(s.clone()),
            LatinStructure::Abelian => classifications.3.push(s.clone()),
        }
    }

    let (q, l, g, a) = classifications;

    // println!("Total amount of latin squares: {}", squares.len());
    // println!("Quasigroups: {}, Loops: {}, Groups: {}, Abelian: {}", q.len(), l.len(), g.len(), a.len());

    let perms = Permutation::generate_all(n);

    let bar = ProgressBar::new(factorial(n) as u64);

    for p in perms {
        bar.inc(1);
        for s in q.iter() {
            let mut test = s.clone();

            test.apply_permutation(p.clone());

            if test.classify() != LatinStructure::Quasigroup {
                println!("q FAILED!");
                panic!();
            }
        }

        for s in l.iter() {
            let mut test = s.clone();

            test.apply_permutation(p.clone());

            if test.classify() != LatinStructure::Loop {
                println!("l FAILED!");
                panic!();
            }
        }

        for s in g.iter() {
            let mut test = s.clone();

            test.apply_permutation(p.clone());

            if test.classify() != LatinStructure::Group {
                println!("g FAILED!");
                panic!();
            }
        }

        for s in a.iter() {
            let mut test = s.clone();

            test.apply_permutation(p.clone());

            if test.classify() != LatinStructure::Abelian {
                println!("a FAILED!");
                panic!();
            }
        }
    }
}
