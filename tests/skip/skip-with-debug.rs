use modular_bitfield::prelude::*;

#[bitfield]
#[derive(Debug)]
pub struct Sparse {
    #[skip(getters)]
    no_getters: B10,
    a: bool,
    #[skip(setters)]
    no_setters: B10,
    b: bool,
    #[skip]
    skipped: B10,
}

fn main() {
    let sparse = Sparse::new()
        .with_a(true)
        .with_b(false);
    assert_eq!(
        format!("{:?}", sparse),
        "Sparse { a: true, no_setters: 0, b: false }",
    );
    assert_eq!(
        format!("{:#X?}", sparse),
        "Sparse {\n    a: true,\n    no_setters: 0x0,\n    b: false,\n}",
    );
}
