pub mod traits;

pub mod latin_square;
pub use latin_square::LatinSquare;

mod permutation;
pub use permutation::Permutation;

mod bits;
pub use bits::Bits;

mod latin_type;
pub use latin_type::LatinType;

pub mod table;
pub use table::Table;

mod sidedness;
pub use sidedness::Sidedness;
pub use sidedness::SIDES;

mod latin_square_classification;
pub use latin_square_classification::LatinSquareClassification;

mod latin_structure;
use latin_structure::LatinStructure;

mod square_information;
pub use square_information::SquareInformation;

mod spreadsheet_colors;
pub use spreadsheet_colors::SpreadsheetColours;

type AffineAutomorphism = (usize, usize, Sidedness);
pub type AllAffineAutomorphisms = (bool, Vec<AffineAutomorphism>);
