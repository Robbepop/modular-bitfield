use modular_bitfield::prelude::*;

#[bitfield]
#[cfg_attr(test, repr(u32))]
pub struct SignedInt {
    sign: bool,
    value: B31,
}

fn main() {
    let i1 = SignedInt::new().with_sign(true).with_value(0x123);
    let i2 = SignedInt::from(0x8000_0123);
    assert_eq!(i1.sign(), i2.sign());
    assert_eq!(i1.value(), i2.value());
}
