#![recursion_limit = "256"]
#![forbid(unsafe_code)]

extern crate proc_macro;

#[macro_use]
mod errors;
mod bitfield;
mod bitfield_specifier;
mod define_specifiers;

use proc_macro::TokenStream;

/// Generates the `B1`, `B2`, ..., `B128` bitfield specifiers.
///
/// Only of use witihn the `modular_bitfield` crate itself.
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
/// Fields are accessed by a method of the same name, except in the case of tuple structs,
/// which use `get_n()` style methods, where `n` is the index of the field to access.
/// Likewise, fields can be set with `set_name()` style methods for normal structs,
/// and `set_n()` style methods for tuple structs.
///
/// ## Example
///
/// ```
/// use modular_bitfield::prelude::*;
///
/// #[bitfield]
/// struct Example {
///     a: B1,      // Uses 1 bit
///     #[bits = 7] // Optional, just asserts that B7 uses exactly 7 bits.
///     b: B7,      // Uses 7 bits
///     c: B24,     // Uses 24 bits
/// }
///
/// #[bitfield]
/// struct TupleExample(B1, B7);
///
/// let mut example = Example::new();
/// assert_eq!(example.a(), 0);
/// assert_eq!(example.b(), 0);
/// assert_eq!(example.c(), 0);
/// example.set_a(1);
/// example.set_b(0b0100_0000);
/// example.set_c(1337);
/// assert_eq!(example.a(), 1);
/// assert_eq!(example.b(), 0b0100_0000);
/// assert_eq!(example.c(), 1337);
///
/// let mut tuple = TupleExample::new();
/// assert_eq!(tuple.get_0(), 0);
/// assert_eq!(tuple.get_1(), 0);
/// tuple.set_0(1);
/// tuple.set_1(0b0100_0000);
/// assert_eq!(tuple.get_0(), 1);
/// assert_eq!(tuple.get_1(), 0b0100_0000);
/// ```
#[proc_macro_attribute]
pub fn bitfield(args: TokenStream, input: TokenStream) -> TokenStream {
    bitfield::analyse_and_expand(args.into(), input.into()).into()
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
/// assert_eq!(example.a(), false); // `false as u8` is 0
/// assert_eq!(example.b(), Mode::Sleep);
/// example.set_a(true);
/// example.set_b(Mode::Awake);
/// assert_eq!(example.a(), true); // `true as u8` is 1
/// assert_eq!(example.b(), Mode::Awake);
/// ```
#[proc_macro_derive(BitfieldSpecifier, attributes(bits))]
pub fn bitfield_specifier(input: TokenStream) -> TokenStream {
    bitfield_specifier::generate(input.into()).into()
}
