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

#[bitfield]
struct F {
    _implicit_skip: u8,
}

#[bitfield(skip(all, convert))]
struct G {
    f: u8,
}

#[bitfield(skip(all, new))]
struct H {
    f: u8,
}

#[bitfield(skip(convert, from_bytes, into_bytes))]
struct I {
    f: u8,
}

fn main() {
    let f = F::new();
    f.implicit_skip();
    f._implicit_skip();
}
