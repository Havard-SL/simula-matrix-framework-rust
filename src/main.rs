#![allow(dead_code)]

mod common;

mod latin_square;
use latin_square::LatinSquare;
use latin_square::LatinStructure;
use latin_square::Permutation;

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

fn main() {
    // Set the dimension of the Latin squares i generate.
    let n = 3;

    // Generate all the n by n latin squares.
    let squares = LatinSquare::generate_all(n);

    // Generate all the permutations on n elements.
    let perms = Permutation::generate_all(n);

    let g = generate_automorphism_table(&squares, &perms);

    let t = latex_generate_fancy_cross_table(&g, perms.len(), squares, perms);

    println!("{}", t);
}