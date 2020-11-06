use modular_bitfield::prelude::*;

#[derive(BitfieldSpecifier)]
#[bits = 2]
pub enum Status {
    Red,
    Green,
    Yellow,
}

#[bitfield(bits = 4)]
#[derive(BitfieldSpecifier)]
pub struct Header {
    is_compact: bool,
    is_secure: bool,
    #[bits = 2]
    pre_status: Status,
}

#[bitfield(bits = 16, bytes = 2, filled = false)]
#[derive(BitfieldSpecifier)]
pub struct PackedData {
    #[bits = 4]
    header: Header,
    body: B9,
    #[bits = 2]
    status: Status,
}

fn main() {
    assert_eq!(<Status as Specifier>::BITS, 2);
    assert_eq!(<Header as Specifier>::BITS, 4);
    assert_eq!(<PackedData as Specifier>::BITS, 16);
}
