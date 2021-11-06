use modular_bitfield::error::InvalidBitPattern;
use modular_bitfield::prelude::*;

#[derive(BitfieldSpecifier, Debug, PartialEq)]
pub enum Two {
    Zero,
    One,
    Two,
    Three,
}
#[derive(BitfieldSpecifier, Debug, PartialEq)]
#[bits = 4]
pub enum Four {
    Zero = 0,
    Fifteen = 15,
}
#[derive(BitfieldSpecifier, Debug, PartialEq)]
#[bits = 5]
pub enum Five {
    Zero = 0,
    ThirtyOne = 31,
}

#[derive(BitfieldSpecifier, Debug, PartialEq)]
#[bits = 2]
pub enum SimpleAdt {
    First(Two, Four),
    Second(Two),
    Third,
    Fourth,
}

#[bitfield]
#[derive(Debug, PartialEq)]
pub struct SimpleAdtBitfield {
    adt: SimpleAdt,
}

#[derive(BitfieldSpecifier, Debug, PartialEq)]
pub enum NestedAdt {
    This(SimpleAdt, Two),
    That { foo: Two, bar: Four },
}

#[bitfield(filled = false)]
pub struct NestedAdtBitfield {
    adt: NestedAdt,
}

#[bitfield(filled = false)]
#[derive(BitfieldSpecifier, Debug, PartialEq, Clone, Copy)]
pub struct RegularBf {
    first: B2,
    second: B5,
}

#[derive(BitfieldSpecifier, Debug, PartialEq)]
pub enum AdtWithBf {
    Has(RegularBf),
    Missing,
}

#[bitfield]
pub struct AdtWithBfBitfield {
    adt: AdtWithBf,
}

#[derive(BitfieldSpecifier)]
#[bits = 2]
pub enum Tags {
    Unused = 0,
    Has = 0b01,
    Missing = 0b10,
}

#[derive(BitfieldSpecifier, Debug, PartialEq)]
#[tag(Tags)]
pub enum Tagged {
    Has(RegularBf),
    Missing,
}

#[bitfield(filled = false)]
pub struct TaggedBitfield {
    adt: Tagged,
}

fn main() {
    assert_eq!(std::mem::size_of::<SimpleAdtBitfield>(), 1);
    let mut simple = SimpleAdtBitfield::new();
    assert_eq!(simple.adt(), SimpleAdt::First(Two::Zero, Four::Zero));
    simple.set_adt(SimpleAdt::Second(Two::Three));
    assert_eq!(simple.adt(), SimpleAdt::Second(Two::Three));
    let fail = SimpleAdtBitfield::from_bytes([0x30]);
    assert_eq!(fail.adt_or_err(), Err(InvalidBitPattern::new(0x30u8)));

    assert_eq!(std::mem::size_of::<NestedAdtBitfield>(), 2);
    let mut nested = NestedAdtBitfield::new();
    assert_eq!(
        nested.adt(),
        NestedAdt::This(SimpleAdt::First(Two::Zero, Four::Zero), Two::Zero)
    );
    nested.set_adt(NestedAdt::That {
        foo: Two::One,
        bar: Four::Fifteen,
    });
    assert_eq!(
        nested.adt(),
        NestedAdt::That {
            foo: Two::One,
            bar: Four::Fifteen
        }
    );

    let inner = RegularBf::from_bytes([0b0111_1111]).unwrap();
    let outer = AdtWithBfBitfield::from_bytes([0b1111_1110]);
    assert_eq!(outer.adt(), AdtWithBf::Has(inner));

    let mut tagged = TaggedBitfield::from_bytes([0, 0]).unwrap();
    assert_eq!(tagged.adt_or_err(), Err(InvalidBitPattern::new(0u16)));
    tagged.set_adt(Tagged::Has(inner));
    assert_eq!(tagged.adt(), Tagged::Has(inner));
    assert_eq!(tagged.bytes, [0b1111_1101, 0b1]);
}
