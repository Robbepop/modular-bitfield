use modular_bitfield::prelude::*;

#[bitfield]
#[derive(Debug)]
pub struct Sparse {
    #[skip(getters)]
    unused_1: B10,
    a: bool,
    #[skip(getters)]
    unused_2: B10,
    b: bool,
    #[skip(getters)]
    unused_3: B10,
}

fn main() {
    let sparse = Sparse::from_bytes([0xFF; 4]);
    sparse.set_unused_1(0);
    assert_eq!(sparse.unused_1(), 0); // ERROR!
}
