/// Tests to check for correct execution of checked setters.

use modular_bitfield::prelude::*;

#[bitfield]
pub struct MyTwoBytes {
    a: B1,
    b: B2,
    c: B13,
}

fn main() {
    let mut bitfield = MyTwoBytes::new();

    // Everything is initialized to zero.
    assert_eq!(bitfield.a(), 0);
    assert_eq!(bitfield.b(), 0);
    assert_eq!(bitfield.c(), 0);

    // Do some invalid manipulations.
    assert_eq!(bitfield.set_a_checked(2), Err(Error::OutOfBounds));
    assert_eq!(bitfield.set_b_checked(4), Err(Error::OutOfBounds));
    assert_eq!(bitfield.set_c_checked(12345), Err(Error::OutOfBounds));

    // Asserts that nothing has changed.
    assert_eq!(bitfield.a(), 0);
    assert_eq!(bitfield.b(), 0);
    assert_eq!(bitfield.c(), 0);

    // Do some valid manipulations.
    assert_eq!(bitfield.set_a_checked(1), Ok(()));
    assert_eq!(bitfield.set_b_checked(3), Ok(()));
    assert_eq!(bitfield.set_c_checked(42), Ok(()));

    // Asserts that the valid manipulation has had effect.
    assert_eq!(bitfield.a(), 1);
    assert_eq!(bitfield.b(), 3);
    assert_eq!(bitfield.c(), 42);
}
