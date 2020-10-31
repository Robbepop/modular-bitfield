use modular_bitfield::prelude::*;

#[bitfield(filled = false)]
#[derive(BitfieldSpecifier, Debug, PartialEq, Eq, Copy, Clone)]
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
    let h2 = base.header();
    assert_eq!(h2, h);
    assert_eq!(h2.a(), 1);
    assert_eq!(h2.b(), 2);
}
