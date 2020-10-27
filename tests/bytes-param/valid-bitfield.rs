use modular_bitfield::prelude::*;

// Just requires exactly 32 bits (4 bytes) as expected.
#[bitfield(bytes = 4)]
pub struct Base {
    a: B2,
    b: B6,
    c: u8,
    d: u16,
}

fn main() {
    assert_eq!(core::mem::size_of::<Base>(), 4)
}
