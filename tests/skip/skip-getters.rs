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
    let mut sparse = Sparse::new();
    assert!(!sparse.a());
    assert!(!sparse.b());
    sparse.set_a(true);
    sparse.set_b(true);
    assert!(sparse.a());
    assert!(sparse.b());

    // Use setters of fields with skipped getters:
    sparse.set_unused_1(0b0011_1111_1111);
    sparse.set_unused_2(0b0011_1111_1111);
    sparse.set_unused_3(0b0011_1111_1111);

    assert_eq!(sparse.into_bytes(), [0xFF; 4]);
}
