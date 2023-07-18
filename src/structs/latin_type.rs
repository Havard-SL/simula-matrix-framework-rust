use std::fmt::Display;

use super::LatinStructure;

#[derive(Clone)]
pub struct LatinType {
    pub structure: LatinStructure,
    pub left_identity: bool,
    pub right_identity: bool,
    pub commutative: bool,
}

impl Display for LatinType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut text: String = "".to_string();

        text.push_str(&self.structure.to_string());

        text.push_str(match self.structure {
            LatinStructure::Loop => {
                if self.commutative {
                    "\nCommutative"
                } else {
                    ""
                }
            }
            LatinStructure::Quasigroup => {
                if self.commutative {
                    "\nCommutative"
                } else if self.left_identity {
                    "\nLeft Identity"
                } else if self.right_identity {
                    "\nRight Identity"
                } else {
                    ""
                }
            }
            _ => "",
        });

        write!(f, "{}", &text)
    }
}
