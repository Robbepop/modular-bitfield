/// Tests to check for correct execution of checked setters.

use modular_bitfield::prelude::*;
use modular_bitfield::error::OutOfBounds;

#[bitfield]
#[derive(Debug, PartialEq)]
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
    assert_eq!(bitfield.set_a_checked(2), Err(OutOfBounds));
    assert_eq!(bitfield.set_b_checked(4), Err(OutOfBounds));
    assert_eq!(bitfield.set_c_checked(12345), Err(OutOfBounds));

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

    // Check the checked with statement throws error
    assert_eq!(MyTwoBytes::new().with_a_checked(2), Err(OutOfBounds));
    assert_eq!(MyTwoBytes::new().with_a_checked(1).unwrap().with_b_checked(4), Err(OutOfBounds));

    // Check that with_checked populates values without touching other fields
    let bitfield = bitfield
        .with_a_checked(0).unwrap()
        .with_b_checked(2).unwrap();

    assert_eq!(bitfield.a(), 0);
    assert_eq!(bitfield.b(), 2);
    assert_eq!(bitfield.c(), 42);
}
