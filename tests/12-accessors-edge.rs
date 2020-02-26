// This test is equivalent to 03-accessors but with some fields spanning across
// byte boundaries. This may or may not already work depending on how your
// implementation has been done so far.
//
//
//     ║  first byte   ║  second byte  ║  third byte   ║  fourth byte  ║
//     ╟───────────────╫───────────────╫───────────────╫───────────────╢
//     ║▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒║
//     ╟─────────────────╫───────────╫─────────────────────────╫───────╢
//     ║        a        ║     b     ║            c            ║   d   ║

use modular_bitfield::prelude::*;

#[bitfield]
pub struct EdgeCaseBytes {
    a: B9,
    b: B6,
    c: B13,
    d: B4,
}

fn main() {
    let mut bitfield = EdgeCaseBytes::new();
    assert_eq!(0, bitfield.a());
    assert_eq!(0, bitfield.b());
    assert_eq!(0, bitfield.c());
    assert_eq!(0, bitfield.d());

    let a = 0b1100_0011_1;
    let b = 0b101_010;
    let c = 0x1675;
    let d = 0b1110;

    bitfield.set_a(a);
    bitfield.set_b(b);
    bitfield.set_c(c);
    bitfield.set_d(d);

    assert_eq!(a, bitfield.a());
    assert_eq!(b, bitfield.b());
    assert_eq!(c, bitfield.c());
    assert_eq!(d, bitfield.d());
}
