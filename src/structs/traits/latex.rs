use super::super::AffineAutomorphism;
use super::super::AllAffineAutomorphisms;
use super::super::LatinSquare;
use super::super::LatinStructure;
use super::super::LatinType;
use super::super::Permutation;
use super::super::Sidedness;
use super::super::SquareInformation;
use super::super::Table;

pub trait LaTeX {
    fn latex(&self) -> String;
}

impl LaTeX for Permutation {
    fn latex(&self) -> String {
        let mut text: String = "\\( \\begin{smallmatrix}\n".to_string();

        text.push_str(&self.0[0].to_string());

        for p in self.0.iter() {
            text.push_str(" & ");
            text.push_str(&p.to_string());
        }

        text.push_str("\n\\end{smallmatrix} \\)");

        text
    }
}

fn max_length<T>(rows: &[Vec<T>]) -> usize {
    let mut longest: usize = 0;

    for r in rows {
        if r.len() > longest {
            longest = r.len();
        }
    }

    longest
}

impl<T: LaTeX> LaTeX for Table<T> {
    // Assume: all rows have same length?
    fn latex(&self) -> String {
        let mut text = "".to_string();

        let n = max_length(&self.table);

        text.push_str("\\begin{longtable}{|");
        text.push_str(&"c|".repeat(n));
        text.push_str("}\\hline\n");

        for (i, row) in self.table.iter().enumerate() {
            text.push_str("    ");

            let mut first_passed = false;

            for element in row.iter() {
                if first_passed {
                    text.push_str(" & ");
                } else {
                    first_passed = true;
                }
                text.push_str(&element.latex());
            }

            text.push_str("\\\\\\hline\n");

            if i == 0 {
                text.push_str("\\endhead")
            }
        }

        text.push_str("\\end{longtable}");

        text
    }
}

impl LaTeX for AffineAutomorphism {
    fn latex(&self) -> String {
        let mut text: String = "\\( ".to_string();

        match self.2 {
            Sidedness::Left => {
                text.push_str(&self.1.to_string());
                text.push_str(" + p_{");
                text.push_str(&self.0.to_string());
                text.push('}')
            }
            Sidedness::Right => {
                text.push_str("p_{");
                text.push_str(&self.0.to_string());
                text.push('}');
                text.push_str(" + ");
                text.push_str(&self.1.to_string());
            }
        };

        text.push_str(" \\)");

        text
    }
}

impl LaTeX for AllAffineAutomorphisms {
    fn latex(&self) -> String {
        let mut text: String = "".to_string();

        let b = !self.1.is_empty();

        match (self.0, b) {
            (false, false) => return "".to_string(),
            (true, false) => text.push_str("\\cellcolor{blue}"),
            (false, true) => text.push_str("\\cellcolor{yellow}"),
            (true, true) => text.push_str("\\cellcolor{green}"),
        }

        text.push_str("\\begin{tabular}{@{}c@{}}\n    ");

        if self.0 {
            text.push('x');
        }

        for affine_automorphism in self.1.iter() {
            text.push_str("\\\\\\hline\n    ");
            text.push_str(&affine_automorphism.latex());
        }

        text.push_str("\n\\end{tabular}");

        text
    }
}

impl LaTeX for LatinStructure {
    fn latex(&self) -> String {
        self.to_string()
    }
}

impl LaTeX for LatinType {
    fn latex(&self) -> String {
        let mut text: String = "".to_string();

        let mut additional_rows: Vec<&str> = vec![];

        text.push_str("\\begin{tabular}{@{}c@{}}\n    ");

        text.push_str(&self.structure.latex());
        text.push_str("\\\\");

        match self.structure {
            LatinStructure::Loop => {
                if self.commutative {
                    additional_rows.push("Commutative");
                }
            }
            LatinStructure::Quasigroup => {
                if self.commutative {
                    additional_rows.push("Commutative");
                } else if self.left_identity {
                    additional_rows.push("Left Identity");
                } else if self.right_identity {
                    additional_rows.push("Right Identity");
                }
            }
            _ => (),
        }

        for row in additional_rows {
            text.push_str("\\hline\n    ");
            text.push_str(row);
        }
        text.push_str("\\end{tabular}");

        text
    }
}

impl LaTeX for LatinSquare {
    fn latex(&self) -> String {
        let mut text: String = "\\( \\begin{smallmatrix}\n".to_string();

        for row in self.0.iter() {
            text.push_str("    ");
            text.push_str(&row.first().unwrap().to_string());

            for v in row.iter().skip(1) {
                text.push_str(" & ");
                text.push_str(&v.to_string());
            }

            text.push_str("\\\\\n");
        }

        text.push_str("\\end{smallmatrix} \\)");

        text
    }
}

impl LaTeX for SquareInformation {
    fn latex(&self) -> String {
        let mut text: String;

        match self {
            Self::Class(class) => {
                text = class.latex();
            }
            Self::LatinSquareIndex(index) => {
                text = "\\( s_{".to_string();
                text.push_str(&index.to_string());
                text.push_str("} \\)");
            }
            Self::LatinSquare(latin_square) => {
                text = latin_square.latex();
            }
            Self::None => {
                text = "".to_string();
            }
            Self::Permutation(p) => text = p.latex(),
            Self::PermutationIndex(i) => {
                text = "\\( p_{".to_string();
                text.push_str(&i.to_string());
                text.push_str("} \\)");
            }
            Self::AllAffineAutomorphisms(a) => text = a.latex(),
            Self::FingerprintIndex(index) => {
                text = "\\( F_{".to_string();
                text.push_str(&index.to_string());
                text.push_str("} \\)");
            }
            Self::AutomorphismAndAffineSums((aut, aff)) => {
                text = format!("Automorphisms: {}\\\\\nAffine Automorphisms: {}", aut, aff);
            }
            Self::Text(t) => text = t.clone(),
        }

        text
    }
}
