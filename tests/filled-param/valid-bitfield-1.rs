use modular_bitfield::prelude::*;

// The bitfield has only 7 bits and therefore is unfilled.
#[bitfield(filled = false)]
pub struct UnfilledBitfield {
    a: B7
}

fn main() {}
