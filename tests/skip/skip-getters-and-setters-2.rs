use modular_bitfield::prelude::*;

#[bitfield]
pub struct Sparse {
    #[skip(getters)]
    #[skip(setters)]
    unused_1: B10,
    a: bool,
    #[skip(setters)]
    #[skip(getters)]
    unused_2: B10,
    b: bool,
    #[skip(getters)]
    #[skip(setters)]
    unused_3: B10,
}

fn main() {}
