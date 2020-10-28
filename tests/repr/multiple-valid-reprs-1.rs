use modular_bitfield::prelude::*;

#[bitfield]
#[repr(C, u32)] // The macro simply ignores `repr(C)`
pub struct SignedInt {
    sign: bool,
    value: B31,
}

fn main() {}
