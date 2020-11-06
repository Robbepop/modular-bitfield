// For some bitfield members, working with them as enums will make more sense to
// the user than working with them as integers. We will require enums that have
// a power-of-two number of variants so that they exhaustively cover a fixed
// range of bits.
//
//     // Works like B3, but getter and setter signatures will use
//     // the enum instead of u8.
//     #[derive(BitfieldSpecifier)]
//     enum DeliveryMode {
//         Fixed = 0b000,
//         Lowest = 0b001,
//         SMI = 0b010,
//         RemoteRead = 0b011,
//         NMI = 0b100,
//         Init = 0b101,
//         Startup = 0b110,
//         External = 0b111,
//     }
//
// For this test case it is okay to require that every enum variant has an
// explicit discriminant that is an integer literal. We will relax this
// requirement in a later test case.
//
// Additionally, enums may support a "bits" attribute which allows to enum to
// have a number of variants that is not a power of two. If the #[bits = N]
// attribute is specified, like so:
//
//     #[derive(BitfieldSpecifier)]
//     #[bits = 4]
//     enum SmallPrime {
//         Two = 0b0010,
//         Three = 0b0011,
//         Five = 0b0101,
//         Seven = 0b0111,
//         Eleven = 0b1011,
//         Thirteen = 0b1101,
//     }
//
// then the number of bits required to represent the struct is coerced to N.
//
//     let mut bitfield = MyBitfield::new();
//     assert_eq!(0, bitfield.small_prime_or_err().unwrap_err().invalid_bytes);
//
//     bitfield.set_small_prime(SmallPrime::Seven);
//     let p = bitfield.small_prime_or_err().unwrap_or(SmallPrime::Two);

use modular_bitfield::error::InvalidBitPattern;
use modular_bitfield::prelude::*;

#[bitfield]
pub struct RedirectionTableEntry {
    acknowledged: bool,
    trigger_mode: TriggerMode,
    delivery_mode: DeliveryMode,
    small_prime: SmallPrime,
    reserved: B3,
    another_small_prime: SmallPrime,
}

#[derive(BitfieldSpecifier, Debug, PartialEq)]
pub enum TriggerMode {
    Edge = 0,
    Level = 1,
}

#[derive(BitfieldSpecifier, Debug, PartialEq)]
pub enum DeliveryMode {
    Fixed = 0b000,
    Lowest = 0b001,
    SMI = 0b010,
    RemoteRead = 0b011,
    NMI = 0b100,
    Init = 0b101,
    Startup = 0b110,
    External = 0b111,
}

#[derive(BitfieldSpecifier, Debug, PartialEq)]
#[bits = 4]
pub enum SmallPrime {
    Two = 0b0010,
    Three = 0b0011,
    Five = 0b0101,
    Seven = 0b0111,
    Eleven = 0b1011,
    Thirteen = 0b1101,
}

fn main() {
    assert_eq!(std::mem::size_of::<RedirectionTableEntry>(), 2);

    // Initialized to all 0 bits.
    let mut entry = RedirectionTableEntry::new();
    assert_eq!(entry.acknowledged(), false);
    assert_eq!(entry.trigger_mode(), TriggerMode::Edge);
    assert_eq!(entry.delivery_mode(), DeliveryMode::Fixed);
    assert_eq!(entry.small_prime_or_err(), Err(InvalidBitPattern{ invalid_bytes: 0 }));
    assert_eq!(entry.small_prime_or_err().unwrap_err().invalid_bytes, 0);

    entry.set_acknowledged(true);
    entry.set_delivery_mode(DeliveryMode::SMI);
    entry.set_small_prime(SmallPrime::Five);
    assert_eq!(entry.acknowledged(), true);
    assert_eq!(entry.trigger_mode(), TriggerMode::Edge);
    assert_eq!(entry.delivery_mode(), DeliveryMode::SMI);
    assert_eq!(entry.small_prime(), SmallPrime::Five);
    assert_eq!(entry.small_prime_or_err(), Ok(SmallPrime::Five));
}
