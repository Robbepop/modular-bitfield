use modular_bitfield::prelude::*;

#[derive(BitfieldSpecifier)]
pub enum Two {
    Zero,
    One,
    Two,
    Three,
}

#[derive(BitfieldSpecifier)]
pub enum Tag {
    First,
    Second,
    ThisIsOk,
    AlsoOk,
}

#[derive(BitfieldSpecifier)]
#[tag(Tag)]
pub enum TagMissing {
    First(Two, Two),
    Second(Two),
    Third,
    Fourth,
}

fn main() {}
