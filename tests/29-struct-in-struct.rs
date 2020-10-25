use modular_bitfield::prelude::*;

#[bitfield(specifier = true)]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Header {
    a: B2,
    b: B3,
}

#[bitfield]
#[derive(Debug, PartialEq, Eq)]
pub struct Base {
    pub header: Header,
    pub rest: B3,
}

fn main() {
    let mut base = Base::new();
    assert_eq!(base.header(), Header::new());
    let h = Header::new().with_a(1).with_b(2);
    base.set_header(h);
    assert_eq!(base.header(), h);
}
