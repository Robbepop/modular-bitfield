use modular_bitfield::prelude::*;

#[bitfield(bits = 32, bytes = 4)]
#[repr(u32)]
pub struct SignInteger {
    sign: bool,
    value: B31,
}

fn main() {}
