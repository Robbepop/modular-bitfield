use modular_bitfield::prelude::*;

// The bitfield has exactly 8 bits and therefore is filled.
#[bitfield(filled = true)]
pub struct UnfilledBitfield {
    a: B8
}

fn main() {}
