use modular_bitfield::prelude::*;

#[bitfield(bits = 32, filled = false)]
#[derive(BitfieldSpecifier)]
pub struct SignIntegerLong {
    sign: bool,
    value: B30,
}

fn main() {
    assert_eq!(<SignIntegerLong as Specifier>::BITS, 32);
}
