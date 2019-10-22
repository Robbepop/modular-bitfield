use modular_bitfield::prelude::*;
use std::convert::TryFrom;

#[bitfield]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ColorEntry {    
    r : B5,    
    g : B5,
    b : B5,
    unused: B1,
}

fn main() {
    for i in 0..=std::u16::MAX {
        let entry = ColorEntry::try_from(&i.to_le_bytes()[..]).unwrap();
        let mut new = ColorEntry::new();
        new.set_r(entry.get_r());
        assert_eq!(new.get_r(), entry.get_r());
        new.set_g(entry.get_g());    
        assert_eq!(new.get_g(), entry.get_g());
        new.set_b(entry.get_b());
        assert_eq!(new.get_b(), entry.get_b());
        new.set_unused(entry.get_unused());
        
        assert_eq!(new.get_r(), entry.get_r());
        assert_eq!(new.get_g(), entry.get_g());
        assert_eq!(new.get_b(), entry.get_b());
        assert_eq!(new.get_unused(), entry.get_unused());
    }
}
