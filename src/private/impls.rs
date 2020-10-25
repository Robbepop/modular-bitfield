use super::{
    Bits,
    FromBits,
    IntoBits,
    PopBits,
    PushBits,
};
use crate::Specifier;
use crate::{
    OutOfBounds,
    InvalidBitPattern,
};

impl Specifier for bool {
    const BITS: usize = 1;
    type Bytes = u8;
    type InOut = bool;

    fn into_bytes(input: Self::InOut) -> Result<Self::Bytes, OutOfBounds> {
        Ok([input as u8])
    }

    fn from_bytes(bytes: Self::Bytes) -> Result<Self::InOut, InvalidBitPattern<Self::Bytes>> {
        match bytes {
            [0] => Ok(false),
            [1] => Ok(true),
            invalid_bytes => Err(InvalidBitPattern { invalid_bytes })
        }
    }
}

macro_rules! impl_specifier_for_primitive {
    ( $( ($prim:ty: $bits:literal) ),* $(,)? ) => {
        $(
            impl Specifier for $prim {
                const BITS: usize = $bits;
                type Bytes = $prim;
                type Face = $prim;

                type Bytes = [::core::primitive::u8; $bits / 8];
                type InOut = $prim;

                fn into_bytes(input: Self::InOut) -> Result<Self::Bytes, OutOfBounds> {
                    Ok(input.to_le_bytes())
                }

                fn from_bytes(bytes: Self::Bytes) -> Result<Self::InOut, InvalidBitPattern<Self::Bytes>> {
                    Ok(<$prim>::from_le_bytes(bytes))
                }
            }
        )*
    };
}
impl_specifier_for_primitive!(
    (u8: 8),
    (u16: 16),
    (u32: 32),
    (u64: 64),
    (u128: 128),
);

impl PopBits for u8 {
    #[inline(always)]
    fn pop_bits(&mut self, amount: u32) -> u8 {
        let orig_ones = self.count_ones();
        debug_assert!(0 < amount && amount <= 8);
        let res = *self & ((0x01_u16.wrapping_shl(amount)).wrapping_sub(1) as u8);
        *self = self.checked_shr(amount).unwrap_or(0);
        debug_assert_eq!(res.count_ones() + self.count_ones(), orig_ones);
        res
    }
}

macro_rules! impl_pop_bits {
    ( $($type:ty),+ ) => {
        $(
            impl PopBits for $type {
                #[inline(always)]
                fn pop_bits(&mut self, amount: u32) -> u8 {
                    let orig_ones = self.count_ones();
                    debug_assert!(0 < amount && amount <= 8);
                    let bitmask = 0xFF >> (8 - amount);
                    let res = (*self & bitmask) as u8;
                    *self = self.checked_shr(amount).unwrap_or(0);
                    debug_assert_eq!(res.count_ones() + self.count_ones(), orig_ones);
                    res
                }
            }
        )+
    };
}
impl_pop_bits!(u16, u32, u64, u128);

macro_rules! impl_push_bits {
    ( $($type:ty),+ ) => {
        $(
            impl PushBits for $type {
                #[inline(always)]
                fn push_bits(&mut self, amount: u32, bits: u8) {
                    let orig_ones = self.count_ones();
                    debug_assert!(0 < amount && amount <= 8);
                    let bitmask = 0xFF >> (8 - amount as u8);
                    *self = self.wrapping_shl(amount) | ((bits & bitmask) as $type);
                    debug_assert_eq!((bits & bitmask).count_ones() + orig_ones, self.count_ones());
                }
            }
        )+
    }
}
impl_push_bits!(u8, u16, u32, u64, u128);

impl FromBits<u8> for bool {
    #[inline(always)]
    fn from_bits(bits: Bits<u8>) -> Self {
        bits.into_raw() != 0
    }
}

impl IntoBits<u8> for bool {
    #[inline(always)]
    fn into_bits(self) -> Bits<u8> {
        Bits(self as u8)
    }
}

macro_rules! impl_wrapper_from_naive {
    ( $($type:ty),* ) => {
        $(
            impl IntoBits<$type> for $type {
                #[inline(always)]
                fn into_bits(self) -> Bits<$type> {
                    Bits(self)
                }
            }

            impl FromBits<$type> for $type {
                #[inline(always)]
                fn from_bits(bits: Bits<$type>) -> Self {
                    bits.into_raw()
                }
            }
        )*
    }
}
impl_wrapper_from_naive!(bool, u8, u16, u32, u64, u128);
