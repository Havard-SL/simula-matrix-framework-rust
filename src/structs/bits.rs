use std::cmp::Ordering;

#[derive(PartialEq, Eq)]
pub struct Bits {
    pub bits: Vec<bool>,
}

impl PartialOrd for Bits {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Bits {
    fn cmp(&self, other: &Self) -> Ordering {
        let n = self.bits.len() - 1;

        for (i, b) in self.bits.iter().rev().enumerate() {
            if b != &other.bits[n - i] {
                if *b {
                    return Ordering::Greater;
                } else {
                    return Ordering::Less;
                }
            }
        }

        Ordering::Equal
    }
}
