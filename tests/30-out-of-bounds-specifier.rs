use modular_bitfield::prelude::*;

#[bitfield(specifier = true, filled = false)]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Header {
    a: B1,
    b: B128,
}

fn main() {}