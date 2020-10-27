use modular_bitfield::prelude::*;

// The bitfield only has 23 bits and therefore is unfilled.
#[bitfield(specifier = true, filled = false)]
pub struct UnfilledSpecifier {
    a: B7,
    b: u16,
}

fn main() {}
