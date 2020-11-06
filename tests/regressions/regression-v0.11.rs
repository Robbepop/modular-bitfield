use modular_bitfield::prelude::*;

#[bitfield(bits = 8)]
#[derive(Copy, Clone)]
pub struct Flags {
    pub a: bool,
    #[skip]
    __: bool,
    pub b: bool,
    #[skip]
    __: B5,
}

fn main() {
    let mut flags = Flags::new();
    assert!(!flags.a());
    assert!(!flags.b());
    assert_eq!(flags.into_bytes(), [0b0000_0000]);
    flags.set_a(true);
    flags.set_b(true);
    assert!(flags.a());
    assert!(flags.b());
    assert_eq!(flags.into_bytes(), [0b0000_0101]);
}
