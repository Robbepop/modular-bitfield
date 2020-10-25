use modular_bitfield::prelude::*;

#[bitfield]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ColorEntry {
    r: B5,
    g: B5,
    b: B5,
    unused: B1,
}

fn main() {
    for i in 0..=std::u16::MAX {
        let entry = ColorEntry::from_bytes(i.to_le_bytes());
        let mut new = ColorEntry::new();
        new.set_r(entry.r());
        assert_eq!(new.r(), entry.r());
        new.set_g(entry.g());
        assert_eq!(new.g(), entry.g());
        new.set_b(entry.b());
        assert_eq!(new.b(), entry.b());
        new.set_unused(entry.unused());

        assert_eq!(new.r(), entry.r());
        assert_eq!(new.g(), entry.g());
        assert_eq!(new.b(), entry.b());
        assert_eq!(new.unused(), entry.unused());
    }
}
