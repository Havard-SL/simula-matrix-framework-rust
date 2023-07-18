use color::Rgb;

pub enum SpreadsheetColours {
    AffineAutomorphism(Rgb<u8>),
    Automorphism(Rgb<u8>),
    AutomorphismAndAffine(Rgb<u8>),
    NoColor,
}