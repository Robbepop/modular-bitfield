// We also want to allow for tuple structs to be accepted by the `#[bitfield]` macro.
//
// For this we generate getters and setters in a way that refer to the corresponding
// number of the field within the annotated tuple struct.

use modular_bitfield::prelude::*;

#[bitfield]
struct MyTwoBytes(bool, B7, B8);

fn main() {
    let mut test = MyTwoBytes::new();

    assert_eq!(test.get_0(), false);
    assert_eq!(test.get_1(), 0);
    assert_eq!(test.get_2(), 0);

    test.set_0(true);
    test.set_1(42);
    test.set_2(0xFF);

    assert_eq!(test.get_0(), true);
    assert_eq!(test.get_1(), 42);
    assert_eq!(test.get_2(), 0xFF);
}
