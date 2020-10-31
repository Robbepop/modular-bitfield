use modular_bitfield::prelude::*;

#[bitfield]
pub struct Sparse {
    #[skip]
    __: B10,
    a: bool,
    #[skip]
    __: B10,
    b: bool,
    #[skip]
    __: B10,
}

fn main() {}
