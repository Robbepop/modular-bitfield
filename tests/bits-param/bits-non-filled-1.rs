use modular_bitfield::prelude::*;

#[bitfield(bits = 32, filled = false)]
#[derive(BitfieldSpecifier)]
pub struct SignIntegerShort {
    sign: bool,
    value: B7,
}

fn main() {
    assert_eq!(<SignIntegerShort as Specifier>::BITS, 32);
}
