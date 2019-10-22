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
//!
//! ### Showcase
//!
//! ```
//! use modular_bitfield::prelude::*;
//!
//! // Works with aliases - just for the showcase.
//! type Vitamin = B12;
//!
//! /// Bitfield struct with 32 bits in total.
//! #[bitfield]
//! #[derive(Debug, PartialEq, Eq)]
//! pub struct Example {
//!     a: bool,         // Uses 1 bit
//!     b: B9,           // Uses 9 bits
//!     c: Vitamin,      // Uses 12 bits, works with aliases.
//!     #[bits = 3]      // Optional, asserts at compiletime that `DeliveryMode` uses 3 bits.
//!     d: DeliveryMode, // Uses 3 bits
//!     e: B7,           // Uses 7 bits
//! }
//!
//! /// Enums that derive from `BitfieldSpecifier`
//! /// can also be used within bitfield structs
//! /// as shown above.
//! #[derive(BitfieldSpecifier, Debug, PartialEq)]
//! pub enum DeliveryMode {
//!     Fixed = 1,
//!     Lowest,
//!     SMI,
//!     RemoteRead,
//!     NMI,
//!     Init = 0,
//!     Startup = 6,
//!     External,
//! }
//!
//! fn main() {
//!     let mut example = Example::new();
//!
//!     // Assert that everything is inizialized to 0.
//!     assert_eq!(example.get_a(), false);
//!     assert_eq!(example.get_b(), 0);
//!     assert_eq!(example.get_c(), 0);
//!     assert_eq!(example.get_d(), DeliveryMode::Init);
//!     assert_eq!(example.get_e(), 0);
//!
//!     // Modify the bitfields.
//!     example.set_a(true);
//!     example.set_b(0b0001_1111_1111_u16); // Uses `u16`
//!     example.set_c(42_u16);           // Uses `u16`
//!     example.set_d(DeliveryMode::Startup);
//!     example.set_e(1);                // Uses `u8`
//!
//!     // Assert the previous modifications.
//!     assert_eq!(example.get_a(), true);
//!     assert_eq!(example.get_b(), 0b0001_1111_1111_u16);
//!     assert_eq!(example.get_c(), 42);
//!     assert_eq!(example.get_d(), DeliveryMode::Startup);
//!     assert_eq!(example.get_e(), 1_u8);
//!
//!     // Safe API allows for better testing
//!     assert_eq!(example.set_e_checked(200), Err(Error::OutOfBounds));
//!
//!     // Can convert from and to bytes.
//!     assert_eq!(example.to_bytes(), &[255, 171, 128, 3]);
//!     use std::convert::TryFrom as _;
//!     let copy = Example::try_from(example.to_bytes()).unwrap();
//!     assert_eq!(example, copy);
//! }
//! ```
//!
//! ## Generated Structure
//!
//! From David Tolnay's procedural macro workshop:
//!
//! The macro conceptualizes given structs as a sequence of bits 0..N.
//! The bits are grouped into fields in the order specified by the struct written by the user.
//!
//! The `#[bitfield]` attribute rewrites the caller's struct into a private byte array representation
//! with public getter and setter methods for each field.
//! The total number of bits N is required to be a multiple of 8: This is checked at compile time.
//!
//! ### Example
//!
//! The following invocation builds a struct with a total size of 32 bits or 4 bytes.
//! It places field `a` in the least significant bit of the first byte,
//! field `b` in the next three least significant bits,
//! field `c` in the remaining four most significant bits of the first byte,
//! and field `d` spanning the next three bytes.
//!
//! ```rust
//! use modular_bitfield::prelude::*;
//!
//! #[bitfield]
//! pub struct MyFourBytes {
//!     a: B1,
//!     b: B3,
//!     c: B4,
//!     d: B24,
//! }
//! ```
//! ```no_compile
//!                                least significant bit of third byte
//!                                  ┊           most significant
//!                                  ┊             ┊
//!                                  ┊             ┊
//! ║  first byte   ║  second byte  ║  third byte   ║  fourth byte  ║
//! ╟───────────────╫───────────────╫───────────────╫───────────────╢
//! ║▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒║
//! ╟─╫─────╫───────╫───────────────────────────────────────────────╢
//! ║a║  b  ║   c   ║                       d                       ║
//!                  ┊                                             ┊
//!                  ┊                                             ┊
//!                least significant bit of d         most significant
//! ```

#![no_std]

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
        Error,
    };
}

/// Error that can be encountered operating on bitfields.
#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    /// A setter received an input that is invalid for the associated bitfield specifier.
    ///
    /// # Example
    ///
    /// Consider a field `a: B2` of a bitfield struct that uses 2 bits.
    /// It having 2 bits the valid bounds of `a` are `0..4`.
    /// The error is returned if a user tries to set its value to a value
    /// that is not within the range `0..4`, e.g. 5.
    OutOfBounds,
    /// Encountered upon using `from_bytes` if too many or too few bytes have been given.
    InvalidBufferLen,
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Error::OutOfBounds => {
                write!(f, "Encountered an out of bounds value")
            }
            Error::InvalidBufferLen => {
                write!(f, "Too many or too few bytes given to construct from bytes")
            }
        }
    }
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
        let orig_bits = self.count_ones();
        debug_assert!(0 < amount && amount <= 8);
        let res = *self & ((0x1_u16.wrapping_shl(amount)).wrapping_sub(1) as u8);
        *self = match self.overflowing_shr(amount) {
            (v, false) => v,
            _ => 0,
        };
        assert_eq!(res.count_ones() + self.count_ones(), orig_bits);
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
                    assert_eq!((bits & (0xFF >> (8 - amount))).count_ones() + orig_bits, self.count_ones());
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
                    assert_eq!(res.count_ones() + self.count_ones(), orig_bits);
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
/// Should generally not be implemented directly by users
/// but through the macros provided by the crate.
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
