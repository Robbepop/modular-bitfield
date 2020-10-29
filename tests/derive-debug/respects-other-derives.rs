use modular_bitfield::prelude::*;

#[bitfield]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Color {
    r: B6,
    g: B6,
    b: B6,
    a: B6,
}

fn main() {
    let color1 = Color::new()
        .with_r(63)
        .with_g(32)
        .with_b(16)
        .with_a(8);
    let color2 = color1.clone();
    assert_eq!(color1, color2);
    assert_eq!(
        format!("{:?}", color1),
        "Color { r: 63, g: 32, b: 16, a: 8 }",
    );
    assert_eq!(
        format!("{:#x?}", color2),
        "Color {\n    r: 0x3f,\n    g: 0x20,\n    b: 0x10,\n    a: 0x8,\n}",
    );
}
