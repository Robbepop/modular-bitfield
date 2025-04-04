use modular_bitfield::prelude::*;

#[bitfield(skip(new, new))]
struct A {
    f: u8,
}

#[bitfield(skip(from_bytes, new, from_bytes))]
struct B {
    f: u8,
}

#[bitfield(skip(new, into_bytes, into_bytes))]
struct C {
    f: u8,
}

#[bitfield(skip(invalid))]
struct D {
    f: u8,
}

#[bitfield(skip)]
struct E {
    f: u8,
}

fn main() {}
