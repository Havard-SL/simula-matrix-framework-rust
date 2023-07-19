use super::super::AffineAutomorphism;
use super::super::AllAffineAutomorphisms;
use super::super::Sidedness;
use super::super::SpreadsheetColours;
use super::super::SquareInformation;

pub trait SpreadsheetDisplay {
    fn spreadsheet_display(&self) -> String;

    fn color(&self) -> SpreadsheetColours;
}

impl SpreadsheetDisplay for AffineAutomorphism {
    fn spreadsheet_display(&self) -> String {
        let mut text: String = "".to_string();

        match self.2 {
            Sidedness::Left => {
                text.push_str(&self.1.to_string());
                text.push_str(" + p_");
                text.push_str(&self.0.to_string());
            }
            Sidedness::Right => {
                text.push_str("p_");
                text.push_str(&self.0.to_string());
                text.push_str(" + ");
                text.push_str(&self.1.to_string());
            }
        };

        text
    }

    fn color(&self) -> SpreadsheetColours {
        SpreadsheetColours::NoColor
    }
}

impl SpreadsheetDisplay for AllAffineAutomorphisms {
    fn spreadsheet_display(&self) -> String {
        let mut text: String = "".to_string();

        if self.0 {
            text.push('x');
        }

        for affine_automorphism in self.1.iter() {
            text.push('\n');
            text.push_str(&affine_automorphism.spreadsheet_display());
        }

        text
    }

    fn color(&self) -> SpreadsheetColours {
        let b = !self.1.is_empty();

        match (self.0, b) {
            (false, false) => SpreadsheetColours::NoColor,
            (true, false) => SpreadsheetColours::Automorphism,
            (false, true) => SpreadsheetColours::AffineAutomorphism,
            (true, true) => SpreadsheetColours::AutomorphismAndAffine,
        }
    }
}

impl SpreadsheetDisplay for SquareInformation {
    fn spreadsheet_display(&self) -> String {
        self.to_string()
    }

    fn color(&self) -> SpreadsheetColours {
        match self {
            SquareInformation::AllAffineAutomorphisms(a) => a.color(),
            SquareInformation::AutomorphismAndAffineSums((aut, aff)) => {
                (aut != &0_usize, vec![(0, 0, Sidedness::Left); *aff]).color()
            }
            _ => SpreadsheetColours::NoColor,
        }
    }
}
