use modular_bitfield::prelude::*;

#[bitfield(specifier = true)]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Header {
    a: B1,
    b: B128,
}

fn main() {}