use modular_bitfield::prelude::*;
use modular_bitfield::error::OutOfBounds;

#[bitfield(filled = false)]
#[derive(BitfieldSpecifier, Debug, PartialEq, Eq, Copy, Clone)]
pub struct Unfilled {
    a: B2,
}

fn main() {
    assert_eq!(Unfilled::from_bytes([0x00]), Ok(Unfilled::new()));
    assert_eq!(Unfilled::from_bytes([0b0000_0001]), Ok(Unfilled::new().with_a(1)));
    assert_eq!(Unfilled::from_bytes([0b0000_0010]), Ok(Unfilled::new().with_a(2)));
    assert_eq!(Unfilled::from_bytes([0b0000_0011]), Ok(Unfilled::new().with_a(3)));
    assert_eq!(Unfilled::from_bytes([0b0000_0100]), Err(OutOfBounds));
}
