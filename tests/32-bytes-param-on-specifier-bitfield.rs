use modular_bitfield::prelude::*;

// Is only 9 bits, so will be 2 bytes in size.
#[bitfield(specifier = true, bytes = 2)]
pub struct Header {
    a: B6,
    b: bool,
    c: bool,
    d: bool,
}

fn main() {
    assert!(core::mem::size_of::<Base>(), 2)
}
