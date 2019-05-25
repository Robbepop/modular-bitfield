// These tests check the conversions from and to bytes.

use modular_bitfield::prelude::*;

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

    assert_eq!(bitfield_1.get_a(), true);
    assert_eq!(bitfield_1.get_b(), 3);
    assert_eq!(bitfield_1.get_c(), 444);
    assert_eq!(bitfield_1.get_d(), 1337);

    let bytes = bitfield_1.to_bytes();
    assert_eq!(bytes, &[231, 13, 57, 5]);

    use std::convert::TryFrom;
    let bitfield2 = MyFourBytes::try_from(bytes).unwrap();

    assert_eq!(bitfield2.get_a(), true);
    assert_eq!(bitfield2.get_b(), 3);
    assert_eq!(bitfield2.get_c(), 444);
    assert_eq!(bitfield2.get_d(), 1337);

    let too_few_bytes = &bytes[0..2];
    let too_many_bytes = {
        let mut bytes = bytes.to_vec();
        bytes.push(0);
        bytes
    };

    assert_eq!(MyFourBytes::try_from(too_few_bytes), Err(Error::InvalidBufferLen));
    assert_eq!(MyFourBytes::try_from(&too_many_bytes[..]), Err(Error::InvalidBufferLen));
}
