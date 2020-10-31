use modular_bitfield::prelude::*;

// Is only 9 bits, so will be 2 bytes in size.
#[bitfield(bytes = 2, filled = false)]
#[derive(BitfieldSpecifier)]
pub struct Header {
    a: B6,
    b: bool,
    c: bool,
    d: bool,
}

fn main() {
    assert_eq!(core::mem::size_of::<Header>(), 2)
}
