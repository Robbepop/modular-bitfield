// Tests bitfield specifiers of more than 64 bit.

use modular_bitfield::prelude::*;

#[bitfield]
pub struct SomeMoreBytes {
    a: B47,
    b: B65,
    c: B128,
}

fn main() {
    let mut bitfield = SomeMoreBytes::new();

    // Everything is initialized to zero.
    assert_eq!(bitfield.a(), 0);
    assert_eq!(bitfield.b(), 0);
    assert_eq!(bitfield.c(), 0);

    // Manipulate bitfield.
    assert_eq!(bitfield.set_a_checked(1), Ok(()));
    assert_eq!(bitfield.set_b_checked(3), Ok(()));
    assert_eq!(bitfield.set_c_checked(42), Ok(()));

    // Check that manipulation was successful.
    assert_eq!(bitfield.a(), 1);
    assert_eq!(bitfield.b(), 3);
    assert_eq!(bitfield.c(), 42);

    // // Manually reset the bitfield.
    bitfield.set_a(0);
    bitfield.set_b(0);
    bitfield.set_c(0);

    // // Check if reset was successful.
    assert_eq!(bitfield.a(), 0);
    assert_eq!(bitfield.b(), 0);
    assert_eq!(bitfield.c(), 0);
}
