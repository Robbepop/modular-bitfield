use super::Bits;
use super::{FromBits, IntoBits, PopBits, PushBits};
use crate::Specifier;

impl Specifier for bool {
    const BITS: usize = 1;
    type Base = u8;
    type Face = bool;
}

impl PopBits for u8 {
    #[inline(always)]
    fn pop_bits(&mut self, amount: u32) -> u8 {
        let orig_bits = self.count_ones();
        debug_assert!(0 < amount && amount <= 8);
        let res = *self & ((0x1_u16.wrapping_shl(amount)).wrapping_sub(1) as u8);
        *self = match self.overflowing_shr(amount) {
            (v, false) => v,
            _ => 0,
        };
        debug_assert_eq!(res.count_ones() + self.count_ones(), orig_bits);
        res
    }
}

macro_rules! impl_push_bits {
    ( $($type:ty),+ ) => {
        $(
            impl PushBits for $type {
                #[inline(always)]
                fn push_bits(&mut self, amount: u32, bits: u8) {
                    let orig_bits = self.count_ones();
                    debug_assert!(0 < amount && amount <= 8);
                    *self = self.wrapping_shl(amount);
                    *self |= (bits & (0xFF >> (8 - amount))) as $type;
                    debug_assert_eq!((bits & (0xFF >> (8 - amount))).count_ones() + orig_bits, self.count_ones());
                }
            }
        )+
    }
}
impl_push_bits!(u8, u16, u32, u64, u128);

macro_rules! impl_pop_bits {
    ( $($type:ty),+ ) => {
        $(
            impl PopBits for $type {
                #[inline(always)]
                fn pop_bits(&mut self, amount: u32) -> u8 {
                    let orig_bits = self.count_ones();
                    debug_assert!(0 < amount && amount <= 8);
                    let res = (*self & (0xFF >> (8 - amount))) as u8;
                    *self = match self.overflowing_shr(amount) {
                        (v, false) => v,
                        _ => 0,
                    };
                    debug_assert_eq!(res.count_ones() + self.count_ones(), orig_bits);
                    res
                }
            }
        )+
    };
}
impl_pop_bits!(u16, u32, u64, u128);

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
