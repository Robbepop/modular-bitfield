use modular_bitfield::prelude::*;

// The bitfield has 24 bits and therefore is filled.
#[bitfield(filled = true)]
#[derive(BitfieldSpecifier)]
pub struct FilledSpecifier {
    a: B8,
    b: u16,
}

fn main() {}
