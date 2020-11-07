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

fn main() {
    let mut sparse = Sparse::new();
    assert!(!sparse.a());
    assert!(!sparse.b());
    sparse.set_a(true);
    sparse.set_b(true);
    assert!(sparse.a());
    assert!(sparse.b());
}
