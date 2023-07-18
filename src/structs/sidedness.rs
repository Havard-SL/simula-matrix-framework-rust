pub const SIDES: [Sidedness; 2] = [Sidedness::Left, Sidedness::Right];

#[derive(Clone, Debug)]
pub enum Sidedness {
    Left,
    Right,
}
