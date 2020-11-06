use modular_bitfield::prelude::*;

#[bitfield(bits = 32, filled = false)]
pub struct SignIntegerShort {
    sign: bool,
    value: B7,
}

#[bitfield(bits = 32, filled = false)]
pub struct SignIntegerLong {
    sign: bool,
    value: B30,
}

fn main() {}
