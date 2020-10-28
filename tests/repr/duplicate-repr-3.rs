use modular_bitfield::prelude::*;

#[bitfield]
#[repr(u32)]
#[cfg_attr(test, repr(u32))]
pub struct SignedInt {
    sign: bool,
    value: B31,
}

fn main() {}
