use modular_bitfield::prelude::*;

// The bitfield has 24 bits and therefore is filled.
#[bitfield(specifier = true, filled = true)]
pub struct UnfilledSpecifier {
    a: B8,
    b: u16,
}

fn main() {}
