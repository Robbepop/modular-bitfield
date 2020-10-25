use modular_bitfield::prelude::*;

#[bitfield(specifier = true)]
pub struct Header {
    a: B2,
    b: B3,
}

#[bitfield]
pub struct Base {
    pub header: Header,
    pub rest: B3,
}

fn main() {
    let base = Base::new();
    assert_eq!(base.header(), Header::new());
    let h = Header::new().with_a(1).with_b(2);
    base.set_header(h);
    assert_eq!(base.header(), h);
}
