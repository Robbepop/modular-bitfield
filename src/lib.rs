//! Provides macros to support bitfield structs allowing for modular use of bit-enums.
//!
//! The mainly provided macros are `#[bitfield]` for structs and
//! `#[derive(BitfieldSpecifier)]` for enums that shall be usable
//! within bitfield structs.
//!
//! There are preset bitfield specifiers such as `B1`, `B2`,..,`B64`
//! that allow for easy bitfield usage in structs very similar to how
//! they work in C or C++.
//!
//! - Performance of the macro generated code is as fast as its hand-written
//!   alternative.
//! - Compile-time checks allow for safe usage of bitfield structs and enums.

pub use modular_bitfield_impl::{
    bitfield,
    BitfieldSpecifier,
};

/// Preset check types and traits used internally.
///
/// # Note
///
/// Do not use entities defined in here directly!
#[doc(hidden)]
pub mod checks;

/// The prelude: `use modular_bitfield::prelude::*;`
pub mod prelude {
    pub use super::{
        specifiers::*,
        PopBits,
        PushBits,
        bitfield,
        BitfieldSpecifier,
        Specifier,
        SpecifierBase,
        IntoBits,
        FromBits,
    };
}

/// The default set of predefined specifiers.
pub mod specifiers {
    modular_bitfield_impl::define_specifiers!();
}

/// Helper trait for underlying primitives handling of bitfields.
///
/// # Note
///
/// Must not and cannot be implemented by dependencies.
#[doc(hidden)]
pub trait PushBits: checks::private::Sealed {
    fn push_bits(&mut self, amount: u32, bits: u8);
}

/// Helper trait for underlying primitives handling of bitfields.
///
/// # Note
///
/// Must not and cannot be implemented by dependencies.
#[doc(hidden)]
pub trait PopBits: checks::private::Sealed {
    fn pop_bits(&mut self, amount: u32) -> u8;
}

macro_rules! impl_sealed_for {
    ( $($primitive:ty),* ) => {
        $(
            impl checks::private::Sealed for $primitive {}
        )*
    }
}

impl_sealed_for!(bool, u8, u16, u32, u64, u128);

impl PopBits for u8 {
    #[inline(always)]
    fn pop_bits(&mut self, amount: u32) -> u8 {
        debug_assert!(amount <= 8);
        let res = *self & ((0x1_u16.wrapping_shl(amount) as u8).wrapping_sub(1));
        *self = self.wrapping_shr(amount);
        res
    }
}

macro_rules! impl_push_bits {
    ( $($type:ty),+ ) => {
        $(
            impl PushBits for $type {
                #[inline(always)]
                fn push_bits(&mut self, amount: u32, bits: u8) {
                    debug_assert!(amount <= 8);
                    *self <<= amount;
                    *self |= (bits & ((0x1_u16.wrapping_shl(amount) as u8).wrapping_sub(1))) as $type;
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
                    debug_assert!(amount <= 8);
                    let res = (*self & ((0x1 << amount) - 1)) as u8;
                    *self >>= amount;
                    res
                }
            }
        )+
    };
}

impl_pop_bits!(u16, u32, u64, u128);

/// Trait implemented by primitives that drive bitfield manipulations generically.
#[doc(hidden)]
pub trait SpecifierBase: checks::private::Sealed {
    /// The base type that the specifier is operating on.
    type Base;
}

/// Trait implemented by all bitfield specifiers.
///
/// # Note
///
/// These can be all unsigned fixed-size primitives,
/// represented by `B1, B2, ... B64` and enums that
/// derive from `BitfieldSpecifier`.
pub trait Specifier {
    /// The amount of bits used by the specifier.
    const BITS: usize;
    /// The base type of the specifier.
    ///
    /// # Note
    ///
    /// This is the type that is used internally for computations.
    type Base:
        Default
        + PushBits
        + PopBits;
    /// The interface type of the specifier.
    ///
    /// # Note
    ///
    /// This is the type that is used for the getters and setters.
    type Face:
        FromBits<Self::Base>
        + IntoBits<Self::Base>;
}

/// Helper struct to convert primitives and enum discriminants.
#[doc(hidden)]
pub struct Bits<T>(pub T);

impl<T> Bits<T> {
    /// Returns the raw underlying representation.
    #[inline(always)]
    pub fn into_raw(self) -> T {
        self.0
    }
}

/// Helper trait to convert to bits.
/// 
/// # Note
///
/// Implemented by primitive specifier types.
#[doc(hidden)]
pub trait IntoBits<T> {
    fn into_bits(self) -> Bits<T>;
}

/// Helper trait to convert from bits.
/// 
/// # Note
///
/// Implemented by primitive specifier types.
#[doc(hidden)]
pub trait FromBits<T> {
    fn from_bits(bits: Bits<T>) -> Self;
}

impl Specifier for bool {
    const BITS: usize = 1;
    type Base = u8;
    type Face = bool;
}

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
