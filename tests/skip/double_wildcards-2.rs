use modular_bitfield::prelude::*;

#[bitfield]
pub struct Sparse {
    #[skip(getters, setters)]
    __: B10,
    a: bool,
    #[skip(getters, setters)]
    __: B10,
    b: bool,
    #[skip(getters, setters)]
    __: B10,
}

fn main() {}
