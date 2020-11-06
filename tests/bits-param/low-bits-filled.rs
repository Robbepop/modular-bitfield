use modular_bitfield::prelude::*;

#[bitfield(bits = 4)]
#[derive(BitfieldSpecifier)]
pub struct Header {
    is_compact: bool,
    is_secure: bool,
    #[bits = 2]
    pre_status: B2,
}

fn main() {
    assert_eq!(<Header as Specifier>::BITS, 4);
}
