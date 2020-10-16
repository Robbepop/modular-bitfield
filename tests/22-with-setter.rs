// Generate getters and.with() setters that manipulate the right range of bits
// corresponding to each field.
//
//
//     ║  first byte   ║  second byte  ║  third byte   ║  fourth byte  ║
//     ╟───────────────╫───────────────╫───────────────╫───────────────╢
//     ║▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒║
//     ╟─╫─────╫───────╫───────────────────────────────────────────────╢
//     ║a║  b  ║   c   ║                       d                       ║

use modular_bitfield::prelude::*;

#[bitfield]
pub struct MyFourBytes {
    a: bool,
    b: B3,
    c: B4,
    d: B24,
}

fn main() {
    let bitfield = MyFourBytes::new()
        .with_a(true)
        .with_b(2)
        .with_c(14)
        .with_d(1_000_000);

    assert_eq!(bitfield.a(), true);
    assert_eq!(bitfield.b(), 2);
    assert_eq!(bitfield.c(), 14);
    assert_eq!(bitfield.d(), 1_000_000);
}
