#![allow(dead_code)]

mod common;

mod latin_square;
use std::fmt::Display;

use latin_square::LatinSquare;
use latin_square::LatinStructure;
use latin_square::Permutation;
use latin_square::Sidedness;
use latin_square::sidedness;

mod abelian;

mod experiments;

// Generate a basic text table from a "sparce" boolean table.
fn generate_cross_table(rows: &Vec<Vec<usize>>, length: usize) -> String {
    let mut text: String = "".to_string();

    let mut previous: Option<usize>;

    for row in rows {
        let mut work: String = "|".to_string();
        previous = None;

        for column in row {
            if let Some(v) = previous {
                work.push_str(&" |".repeat(column - v - 1));
            }
            previous = Some(*column);
            work.push_str("x|");
        }

        work.push_str(&" |".repeat(length - 1 - previous.unwrap()));
        work.push('\n');

        text.push_str(&work);
    }

    text
}

fn latex_matrix(rows: &LatinSquare) -> String {

    let mut text: String = "\\( \\begin{smallmatrix}\n".to_string();

    for row in &rows.0 {
        text.push_str("    ");
        text.push_str(&row[0].to_string());
        for r in row.iter().skip(1) {
            text.push_str(" & ");
            text.push_str(&r.to_string());
        }
        text.push_str(" \\\\\n")
    }
    text.push_str("\\end{smallmatrix} \\)\n");

    text
}

fn latex_permutation(rows: &Permutation) -> String {
    let mut text: String = "\\( \\begin{smallmatrix}\n".to_string();

    text.push_str("    ");
    text.push_str(&0.to_string());
    for r in 1..rows.0.len() {
        text.push_str(" & ");
        text.push_str(&r.to_string());
    }
    text.push_str(" \\\\\n");

    text.push_str("    ");

    text.push_str(&rows.0[0].to_string());
    for r in rows.0.iter().skip(1) {
        text.push_str(" & ");
        text.push_str(&r.to_string());
    }

    text.push_str(" \\\\\n");

    text.push_str("\\end{smallmatrix} \\)\n");

    text
}

// Generate a latex table from a "sparce" boolean table.
fn latex_generate_cross_table(rows: &Vec<Vec<usize>>, length: usize) -> String {
    let mut text: String = "\\begin{tabular}{".to_string();
    text.push_str(&"| c ".repeat(length));
    text.push_str("|} \\hline\n");

    let mut previous: Option<usize>;

    for row in rows {
        let mut work: String = "    ".to_string();
        previous = None;

        for column in row {
            if let Some(v) = previous {
                work.push_str(&"   &".repeat(column - v - 1));
            }
            previous = Some(*column);
            if *column != length - 1 {
                work.push_str(" x &");
            } else {
                work.push_str(" x");
            }
        }
        if previous.unwrap() != length - 1 {
            work.push_str(&"   &".repeat(length - 2 - previous.unwrap()));
            work.push_str("  ");
        }
        work.push_str(" \\\\ \\hline\n");

        text.push_str(&work);
    }
    text.push_str("\\end{tabular}");

    text
}

fn latex_generate_fancy_cross_table(rows: &[Vec<usize>], length: usize, squares: Vec<LatinSquare>, perms: Vec<Permutation>) -> String {
    let mut text: String = "\\begin{longtable}{".to_string();
    text.push_str(&"| c ".repeat(length + 2));
    text.push_str("|} \\hline\n");

    let mut previous: Option<usize>;

    text.push_str(" Class & Latin Square");

    for p in &perms {
        text.push_str("& ");
        text.push_str(&latex_permutation(p))
    }
    text.push_str(" \\\\ \\hline\n\\endhead\n");

    for (i, row) in rows.iter().enumerate() {
        let mut work: String = "    ".to_string();

        let class = match squares[i].classify() {
            LatinStructure::Quasigroup => "Quasigroup",
            LatinStructure::Loop => "\\rowcolor{lime} Loop",
            LatinStructure::Group => "Group",
            LatinStructure::Abelian => "\\rowcolor{cyan} Abelian",
        };

        work.push_str(class);

        // if row == &vec![0] {
        //     work.push_str(" \\rowcolor{gray} ");
        // }

        work.push_str(" &\n");

        work.push_str(&latex_matrix(&squares[i]));

        work.push_str(" & ");

        previous = None;

        for column in row {
            if let Some(v) = previous {
                work.push_str(&"   &".repeat(column - v - 1));
            }
            previous = Some(*column);
            if *column != length - 1 {
                work.push_str(" \\cellcolor[HTML]{AA0044} x &");
            } else {
                work.push_str(" \\cellcolor[HTML]{AA0044} x");
            }
        }
        if previous.unwrap() != length - 1 {
            work.push_str(&"   &".repeat(length - 2 - previous.unwrap()));
            work.push_str("  ");
        }
        work.push_str(" \\\\ \\hline\n");

        text.push_str(&work);
    }
    text.push_str("\\end{longtable}");

    text
}

// Check and print if there are any permutations that are not an automorphism for any latin square.
// Check and print if there are any latin squares that only has the identity as an automorphism.
fn try_automorphism_groups_porperties(
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

// Takes a vec of latin squares, and a vec of permutations and creates the sparse bool table
// where a permutation is an automorphism for a latin square.
fn generate_automorphism_table(squares: &[LatinSquare], perms: &[Permutation]) -> Vec<Vec<usize>> {
    let mut automorphisms_given_group: Vec<Vec<usize>> = vec![];

    // Iterate over every latin square.
    for s in squares.iter() {
        let mut row: Vec<usize> = vec![];

        // Iterate over every permutation
        for (i, p) in perms.iter().enumerate() {
            let mut w = s.clone();

            // Apply the permutation to the latin square.
            w.apply_permutation(p.clone());

            // If the resulting latin square is equal to the starting latin square, then add the (index of the) permutation as an automorphism for that latin square.
            if w == *s {
                row.push(i)
            }
        }
        automorphisms_given_group.push(row);
    }

    automorphisms_given_group
}

type AffineAutomorphism = (usize, usize, Sidedness);
type AllAffineAutomorphisms = (bool, Vec<AffineAutomorphism>);
type LatinSquareClassification = (usize, LatinStructure, Vec<AllAffineAutomorphisms>);

fn generate_cross_table_2(table: &[LatinSquareClassification]) -> String {
    let mut text = "".to_string();

    let mut border = "-".repeat((table[0].2.len() + 2)*14+ 1);
    border.push('\n');


    text.push_str(&border);

    text.push_str(&"|             ".repeat(2));
    text.push('|');

    for i in 0..table[0].2.len() {
        text.push_str("     p_");
        text.push_str(&i.to_string());
        if i < 10 {
            text.push_str("     |");
        } else {
            text.push_str("    |");
        }
    }
    text.push('\n');
    text.push_str(&border);
    
    for r in table.iter() {

        let mut height = 1;

        for p in &r.2 {
            let mut working_height = 1;

            working_height += p.1.len();

            if working_height > height {
                height = working_height;
            }
        }

        text.push_str("|     s_");
        text.push_str(&r.0.to_string());

        if r.0 < 10 {
            text.push_str("     |");
        } else {
            text.push_str("    |");
        }

        let t = match r.1 {
            LatinStructure::Quasigroup => " Quasigroup  |",
            LatinStructure::Loop => " Loop        |",
            LatinStructure::Group => " Group       |",
            LatinStructure::Abelian => " Abelian     |",
        };

        text.push_str(t);

        for w in &r.2 {
            if w.0 {
                text.push_str("      x");
            } else {
                text.push_str("       ");
            }
            text.push_str("      |");
        }
        text.push('\n');
        
        for i in 0..(height - 1) {
            text.push_str("|             |             |");
            for w in &r.2 {
                if let Some(x) = w.1.get(i) {
                    text.push_str("  ");
                    match x.2 {
                        Sidedness::Left => {
                            if x.1 < 10 {
                                text.push(' ');
                            }
                            text.push_str(&x.1.to_string());
                            text.push_str(" + p_");
                            text.push_str(&x.0.to_string());
                            if x.0 < 10 {
                                text.push(' ');
                            }
                        }
                        Sidedness::Right => {
                            if x.0 < 10 {
                                text.push(' ');
                            }
                            text.push_str("p_");
                            text.push_str(&x.0.to_string());
                            text.push_str(" + ");
                            text.push_str(&x.1.to_string());
                            if x.1 < 10 {
                                text.push(' ');
                            }
                        }
                    };
                    text.push_str("  |")
                } else {
                    text.push_str("             |")
                }
            }
            text.push('\n');
        }
        text.push_str(&border);
    }

    text
}

fn calculate_fingerprint(classification: &LatinSquareClassification) -> usize {

    let mut fingerprint: usize = match classification.1 {
        LatinStructure::Quasigroup => 3,
        LatinStructure::Loop => 2,
        LatinStructure::Group => 1,
        LatinStructure::Abelian => 0,    
    };

    for (i, c) in classification.2.iter().enumerate() {
        if c.0 {
            fingerprint += 2_usize.pow((i + 2).try_into().unwrap());
        }
    }

    fingerprint
}

fn print_affine_automorphism_table(squares: &[LatinSquare], perms: &[Permutation]) {
    let mut result: Vec<LatinSquareClassification> = vec![];

    for (j, s) in squares.iter().enumerate() {
        let mut row: LatinSquareClassification = (j, s.classify(), vec![(false, vec![]); perms.len()]);

        for (i, p) in perms.iter().enumerate() {
            let mut w = s.clone();
            w.apply_permutation(p.clone());

            if w == *s {

                row.2[i].0 = true;
                for v in 0..squares[0].0.len() {

                    for side in sidedness::SIDES {
                        let affine_automorphism = s.addition_permutation(v, &side).compose(p);
                        let found_permutation = perms.iter().position(|x| x == &affine_automorphism).unwrap();
                        row.2[found_permutation].1.push((i, v, side));
                    }
                }
            }
        }

        result.push(row);
    }

    result.sort_by_cached_key(calculate_fingerprint);

    let text = generate_cross_table_2(&result);

    println!("{}", &text);
}

pub trait LaTeXIfy {
    fn latex(&self) -> String;
}

type Row<S> = Vec<S>;

// A table, where left[i] + right[i] becomes one row in the table.
// Split it up in order to make the abstraction cleaner and maybe slightly faster?
struct Table<L, R> {
    left: Vec<Row<L>>,
    right: Vec<Row<R>>,
}

impl<L: Display, R: Display> Display for Table<L, R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl<L: LaTeXIfy, R: LaTeXIfy> LaTeXIfy for Table<L, R>  {
    fn latex(&self) -> String {
        todo!()
    }
}

fn main() {
    // Set the dimension of the Latin squares i generate.
    let n = 3;

    // Generate all the n by n latin squares.
    let squares = LatinSquare::generate_all(n);

    // Generate all the permutations on n elements.
    let perms = Permutation::generate_all(n);

    print_affine_automorphism_table(&squares, &perms);
}