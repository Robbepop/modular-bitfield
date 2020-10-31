use modular_bitfield::prelude::*;

#[bitfield]
#[derive(Debug)]
pub struct Sparse {
    #[skip(setters)]
    unused_1: B10,
    a: bool,
    #[skip(setters)]
    unused_2: B10,
    b: bool,
    #[skip(setters)]
    unused_3: B10,
}

fn main() {}
