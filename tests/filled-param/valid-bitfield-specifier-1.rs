use modular_bitfield::prelude::*;

// The bitfield only has 23 bits and therefore is unfilled.
#[bitfield(filled = false)]
#[derive(BitfieldSpecifier)]
pub struct UnfilledSpecifier {
    a: B7,
    b: u16,
}

fn main() {}
