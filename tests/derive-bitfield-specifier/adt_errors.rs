use modular_bitfield::prelude::*;

#[derive(BitfieldSpecifier)]
pub enum Two {
    Zero,
    One,
    Two,
    Three,
}

#[derive(BitfieldSpecifier)]
#[bits = 1]
pub enum OutOfRange {
    First(Two, Two),
    Second(Two),
    OutOfRange,
}

#[derive(BitfieldSpecifier)]
pub enum NonPowerOf2 {
    First(Two, Two),
    Second(Two),
    Third,
}

fn main() {}
