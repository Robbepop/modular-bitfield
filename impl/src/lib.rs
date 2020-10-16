#![recursion_limit = "256"]

extern crate proc_macro;

mod ident_ext;
#[macro_use]
mod errors;
mod bitfield;
mod bitfield_specifier;
mod define_specifiers;

use proc_macro::TokenStream;

#[proc_macro]
pub fn define_specifiers(input: TokenStream) -> TokenStream {
    define_specifiers::generate(input.into()).into()
}

/// Attribute applicable to structs that turns them into bitfield structs.
///
/// Generates getters and setters for all fields in the struct.
/// Can be used modularily in combination with enums that derive from `BitfieldSpecifier`
/// via `#[derive(BitfieldSpecifier)].
///
/// Also generates a simple constructor `new` that initializes all bits to `0`.
///
/// It is possible to attach `#[bits = N]` attribute to struct fields to assert that
/// they are of size N bits.
///
/// ## Example
///
/// ```
/// use modular_bitfield::prelude::*;
///
/// #[bitfield]
/// struct Example {
///     a: B1,  // Uses 1 bit
///     #[bits = 7] // Optional, just asserts that B7 uses exactly 7 bits.
///     b: B7,  // Uses 7 bits
///     c: B24, // Uses 24 bits
/// }
///
/// let mut example = Example::new();
/// assert_eq!(example.get_a(), 0);
/// assert_eq!(example.get_b(), 0);
/// assert_eq!(example.get_c(), 0);
/// example.set_a(1);
/// example.set_b(0b0100_0000);
/// example.set_c(1337);
/// assert_eq!(example.get_a(), 1);
/// assert_eq!(example.get_b(), 0b0100_0000);
/// assert_eq!(example.get_c(), 1337);
/// ```
#[proc_macro_attribute]
pub fn bitfield(args: TokenStream, input: TokenStream) -> TokenStream {
    bitfield::generate(args.into(), input.into()).into()
}

/// Derive macro for enums.
///
/// Generates code for enums to make them usable within `#[bitfield]` structs.
/// Performs compile-time checks to validate that the enum is usable as bitfield specifier.
///
/// ## Example
///
/// ```
/// use modular_bitfield::prelude::*;
///
/// #[bitfield]
/// struct Example {
///     a: bool, // Uses 1 bit
///     b: Mode, // Has 4 variants => uses 2 bits
///     c: B5,   // Uses 5 bits
///     d: B24,  // Uses 24 bits
/// }
///
/// #[derive(BitfieldSpecifier, Debug, PartialEq, Eq)]
/// pub enum Mode {
///     Sleep,
///     Awake,
///     Working,
///     Lazy,
/// }
///
/// let mut example = Example::new();
/// assert_eq!(example.get_a(), false); // `false as u8` is 0
/// assert_eq!(example.get_b(), Mode::Sleep);
/// example.set_a(true);
/// example.set_b(Mode::Awake);
/// assert_eq!(example.get_a(), true); // `true as u8` is 1
/// assert_eq!(example.get_b(), Mode::Awake);
/// ```
#[proc_macro_derive(BitfieldSpecifier)]
pub fn bitfield_specifier(input: TokenStream) -> TokenStream {
    bitfield_specifier::generate(input.into()).into()
}
