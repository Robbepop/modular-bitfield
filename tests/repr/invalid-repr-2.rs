use modular_bitfield::prelude::*;

#[bitfield]
#[cfg_attr(test, repr(invalid))]
pub struct SignedInt {
    sign: bool,
    value: B31,
}

fn main() {}
