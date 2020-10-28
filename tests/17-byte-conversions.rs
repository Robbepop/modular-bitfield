// These tests check the conversions from and to bytes.

use modular_bitfield::prelude::*;
// use std::convert::TryFrom;

#[bitfield]
#[derive(PartialEq, Eq, Debug)]
pub struct MyFourBytes {
    a: bool,
    b: B2,
    c: B13,
    d: B16,
}

fn main() {
    let mut bitfield_1 = MyFourBytes::new();

    bitfield_1.set_a(true);
    bitfield_1.set_b(0b11);
    bitfield_1.set_c(444);
    bitfield_1.set_d(1337);

    assert_eq!(bitfield_1.a(), true);
    assert_eq!(bitfield_1.b(), 3);
    assert_eq!(bitfield_1.c(), 444);
    assert_eq!(bitfield_1.d(), 1337);

    let bytes = bitfield_1.into_bytes().clone();
    assert_eq!(bytes, [231, 13, 57, 5]);

    let bitfield2 = MyFourBytes::from_bytes(bytes);

    assert_eq!(bitfield2.a(), true);
    assert_eq!(bitfield2.b(), 3);
    assert_eq!(bitfield2.c(), 444);
    assert_eq!(bitfield2.d(), 1337);
}
