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
//! /// Tuple structs can also be used as bitfields.
//! #[bitfield]
//! pub struct TupleStruct(bool, B4, DeliveryMode);
//!
//! let mut example = Example::new();
//!
//! // Assert that everything is inizialized to 0.
//! assert_eq!(example.a(), false);
//! assert_eq!(example.b(), 0);
//! assert_eq!(example.c(), 0);
//! assert_eq!(example.d(), DeliveryMode::Init);
//! assert_eq!(example.e(), 0);
//!
//! // Modify the bitfields.
//! example.set_a(true);
//! example.set_b(0b0001_1111_1111_u16);  // Uses `u16`
//! example.set_c(42_u16);                // Uses `u16`
//! example.set_d(DeliveryMode::Startup);
//! example.set_e(1);                     // Uses `u8`
//!
//! // Assert the previous modifications.
//! assert_eq!(example.a(), true);
//! assert_eq!(example.b(), 0b0001_1111_1111_u16);
//! assert_eq!(example.c(), 42);
//! assert_eq!(example.d(), DeliveryMode::Startup);
//! assert_eq!(example.e(), 1_u8);
//!
//! // Safe API allows for better testing
//! assert_eq!(example.set_e_checked(200), Err(Error::OutOfBounds));
//!
//! // Can convert from and to bytes.
//! assert_eq!(example.as_bytes(), &[255, 171, 128, 3]);
//! let copy = unsafe { Example::from_bytes_unchecked(example.as_bytes().clone()) };
//! assert_eq!(example, copy);
//!
//! // Accessing fields of a tuple struct bitfield
//! // uses the `get_n()` and `set_n()` functions.
//! let mut tuple_example = TupleStruct::new();
//! assert_eq!(tuple_example.get_0(), false);
//! assert_eq!(tuple_example.get_1(), 0);
//! assert_eq!(tuple_example.get_2(), DeliveryMode::Init);
//! tuple_example.set_2(DeliveryMode::Fixed);
//! assert_eq!(tuple_example.get_2(), DeliveryMode::Fixed);
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

#[doc(hidden)]
pub mod private;

mod error;
pub use self::error::Error;

/// The prelude: `use modular_bitfield::prelude::*;`
pub mod prelude {
    pub use super::{
        bitfield,
        error::Error,
        specifiers::*,
        BitfieldSpecifier,
        Specifier,
    };
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
    type Base: Default + private::PushBits + private::PopBits;

    /// The interface type of the specifier.
    ///
    /// # Note
    ///
    /// This is the type that is used for the getters and setters.
    type Face: private::FromBits<Self::Base>
        + private::IntoBits<Self::Base>;

    type Bytes;
    type InOut;

    fn into_bytes(input: Self::InOut) -> Result<Self::Bytes, OutOfBounds>;
    fn from_bytes(bytes: Self::Bytes) -> Result<Self::InOut, InvalidBitPattern<Self::Bytes>>;
}

pub struct OutOfBounds;
pub struct InvalidBitPattern<Bytes> {
    pub invalid_bytes: Bytes,
}

/// The default set of predefined specifiers.
pub mod specifiers {
    ::modular_bitfield_impl::define_specifiers!();
}
