use modular_bitfield::prelude::*;

#[bitfield(bits = 32, filled = false)]
pub struct SignInteger {
    sign: bool,
    value: B7,
}

fn main() {}
