// Tests if it is possible to manually reset the bitfields again.

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

    // Manipulate bitfield.
    bitfield.set_a(1);
    bitfield.set_b(3);
    bitfield.set_c(42);

    // Check that manipulation was successful.
    assert_eq!(bitfield.a(), 1);
    assert_eq!(bitfield.b(), 3);
    assert_eq!(bitfield.c(), 42);

    // Manually reset the bitfield.
    bitfield.set_a(0);
    bitfield.set_b(0);
    bitfield.set_c(0);

    // Check if reset was successful.
    assert_eq!(bitfield.a(), 0);
    assert_eq!(bitfield.b(), 0);
    assert_eq!(bitfield.c(), 0);
}
