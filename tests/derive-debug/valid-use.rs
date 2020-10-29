use modular_bitfield::prelude::*;

#[bitfield]
#[derive(Debug)]
pub struct Color {
    r: B6,
    g: B6,
    b: B6,
    a: B6,
}

fn main() {
    let color = Color::new()
        .with_r(63)
        .with_g(32)
        .with_b(16)
        .with_a(8);
    assert_eq!(
        format!("{:?}", color),
        "Color { r: 63, g: 32, b: 16, a: 8 }",
    );
    assert_eq!(
        format!("{:#x?}", color),
        "Color {\n    r: 0x3f,\n    g: 0x20,\n    b: 0x10,\n    a: 0x8,\n}",
    );
}
