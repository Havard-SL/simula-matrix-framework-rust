use super::AllAffineAutomorphisms;
use super::Bits;
use super::LatinSquare;
use super::LatinStructure;
use super::LatinType;

#[derive(Clone)]
pub struct LatinSquareClassification {
    pub class: LatinType,
    pub index: usize,
    pub square: LatinSquare,
    pub all_permutations_all_affine_automorphisms: Vec<AllAffineAutomorphisms>,
}

impl LatinSquareClassification {
    pub fn fingerprint(&self) -> Bits {
        let mut fingerprint: Vec<bool> = match self.class.structure {
            LatinStructure::Quasigroup => vec![false, false],
            LatinStructure::Loop => vec![true, false],
            LatinStructure::Group => vec![false, true],
            LatinStructure::Abelian => vec![true, true],
        };

        for c in self.all_permutations_all_affine_automorphisms.iter() {
            fingerprint.push(c.0);
        }

        Bits { bits: fingerprint }
    }
}
