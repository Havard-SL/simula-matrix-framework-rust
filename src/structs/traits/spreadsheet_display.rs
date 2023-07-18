use super::super::AffineAutomorphism;
use super::super::AllAffineAutomorphisms;
use super::super::Sidedness;

pub trait SpreadsheetDisplay {
    fn spreadsheet_display(&self) -> String;
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
}

impl SpreadsheetDisplay for AllAffineAutomorphisms {
    fn spreadsheet_display(&self) -> String {
        let mut text: String = "".to_string();

        // let b = !self.1.is_empty();

        // match (self.0, b) {
        //     (false, false) => return "".to_string(),
        //     (true, false) => text.push_str("\\cellcolor{blue}"),
        //     (false, true) => text.push_str("\\cellcolor{yellow}"),
        //     (true, true) => text.push_str("\\cellcolor{green}"),
        // }

        if self.0 {
            text.push('x');
        }

        for affine_automorphism in self.1.iter() {
            text.push('\n');
            text.push_str(&affine_automorphism.spreadsheet_display());
        }

        text
    }
}
