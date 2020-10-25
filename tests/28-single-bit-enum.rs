// Validates that in a degenerate case with a single bit, non-power-of-two enums
// behave as expected.

use modular_bitfield::error::InvalidBitPattern;
use modular_bitfield::prelude::*;

#[bitfield]
pub struct UselessStruct {
    reserved: B3,
    field: ForciblyTrue,
    reserved2: B4,
}

#[derive(BitfieldSpecifier, Debug, PartialEq)]
#[bits = 1]
pub enum ForciblyTrue {
    True = 1,
}

fn main() {
    assert_eq!(std::mem::size_of::<UselessStruct>(), 1);

    // Initialized to all 0 bits.
    let entry = UselessStruct::new();
    assert_eq!(entry.field_or_err(), Err(InvalidBitPattern{ invalid_bytes: 0 }));

    let entry = UselessStruct::new().with_field(ForciblyTrue::True);
    assert_eq!(entry.field_or_err(), Ok(ForciblyTrue::True));
}
