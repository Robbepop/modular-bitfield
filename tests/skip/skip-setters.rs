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

fn main() {
    let mut sparse = Sparse::new();
    assert!(!sparse.a());
    assert!(!sparse.b());
    sparse.set_a(true);
    sparse.set_b(true);
    assert!(sparse.a());
    assert!(sparse.b());

    // Use setters of fields with skipped getters:
    assert_eq!(sparse.unused_1(), 0);
    assert_eq!(sparse.unused_2(), 0);
    assert_eq!(sparse.unused_3(), 0);
}
