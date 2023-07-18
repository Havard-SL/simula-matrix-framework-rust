use std::fmt::Display;

// The different classes that a latin square can belong to.
#[derive(Debug, Clone, PartialEq)]
pub enum LatinStructure {
    Quasigroup,
    Loop,
    Group,
    Abelian,
}

impl Display for LatinStructure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            LatinStructure::Quasigroup => "Quasigroup",
            LatinStructure::Loop => "Loop",
            LatinStructure::Group => "Group",
            LatinStructure::Abelian => "Abelian",
        };

        write!(f, "{}", text)
    }
}
