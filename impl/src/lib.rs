#![recursion_limit = "256"]
#![forbid(unsafe_code)]

extern crate proc_macro;

#[macro_use]
mod errors;
mod bitfield;
mod bitfield_attr;
mod bitfield_specifier;
mod config;
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

/// Derive macro for Rust `enums` to implement `Specifier` trait.
///
/// This allows such an enum to be used as a field of a `#[bitfield]` struct.
/// The annotated enum must not have any variants with associated data and
/// by default must have a number of variants that is equal to the power of 2.
///
/// If a user wants to circumvent the latter restriction they can add
/// `#[bits = N]` below the `#[derive(BitfieldSpecifier)]` line in order to
/// signal to the code generation that the enum may have a relaxed number
/// of variants.
///
/// # Example
///
/// ## Example: Basic Usage
///
/// In the following we define a `MaybeWeekday` enum that lists all weekdays
/// as well as an invalid day so that we have a power-of-two number of variants.
///
/// ```
/// use modular_bitfield::prelude::*;
///
/// #[derive(BitfieldSpecifier)]
/// pub enum Weekday {
///     Monday, Tuesday, Wednesday, Thursday, Friday, Saturday, Sunday, None
/// }
/// ```
///
/// ## Example: `#[bits = N]`
///
/// If we want to get rid of the `None` variant we need to add `#[bits = 3]`:
///
/// ```
/// # use modular_bitfield::prelude::*;
/// #
/// #[derive(BitfieldSpecifier)]
/// #[bits = 3]
/// pub enum Weekday {
///     Monday, Tuesday, Wednesday, Thursday, Friday, Saturday, Sunday
/// }
/// ```
///
/// ## Example: Discriminants
///
/// It is possible to explicitly assign discriminants to some of the days.
/// In our case this is useful since our week starts at sunday:
///
/// ```
/// # use modular_bitfield::prelude::*;
/// #
/// #[derive(BitfieldSpecifier)]
/// #[bits = 3]
/// pub enum Weekday {
///     Monday = 1,
///     Tuesday = 2,
///     Wednesday = 3,
///     Thursday = 4,
///     Friday = 5,
///     Saturday = 6,
///     Sunday = 0,
/// }
/// ```
///
/// ## Example: Use in `#[bitfield]`
///
/// Given the above `Weekday` enum that starts at `Sunday` and uses 3 bits in total
/// we can now use it in a `#[bitfield]` annotated struct as follows:
///
/// ```
/// # use modular_bitfield::prelude::*;
/// #
/// # #[derive(BitfieldSpecifier)]
/// # #[bits = 3]
/// # pub enum Weekday {
/// #     Monday = 1,
/// #     Tuesday = 2,
/// #     Wednesday = 3,
/// #     Thursday = 4,
/// #     Friday = 5,
/// #     Saturday = 6,
/// #     Sunday = 0,
/// # }
/// #[bitfield]
/// pub struct MeetingTimeSlot {
///     day: Weekday,
///     from: B6,
///     to: B6,
///     expired: bool,
/// }
/// ```
///
/// The above `MeetingTimeSlot` uses exactly 16 bits and defines our `Weekday` enum as
/// compact `day` bitfield. The `from` and `to` require 6 bits each and finally the
/// `expired` flag requires a single bit.
///
/// ## Example: Interacting
///
/// A user can interact with the above `MeetingTimeSlot` and `Weekday` definitions in
/// the following ways:
///
/// ```
/// # use modular_bitfield::prelude::*;
/// #
/// # #[derive(BitfieldSpecifier, Debug, PartialEq)]
/// # #[bits = 3]
/// # pub enum Weekday {
/// #     Monday = 1,
/// #     Tuesday = 2,
/// #     Wednesday = 3,
/// #     Thursday = 4,
/// #     Friday = 5,
/// #     Saturday = 6,
/// #     Sunday = 0,
/// # }
/// # #[bitfield]
/// # pub struct MeetingTimeSlot {
/// #     day: Weekday,
/// #     from: B6,
/// #     to: B6,
/// #     expired: bool,
/// # }
/// #
/// let mut slot = MeetingTimeSlot::new()
///     .with_day(Weekday::Friday)
///     .with_from(14) // 14:00 CEST
///     .with_to(15); // 15:00 CEST
/// assert_eq!(slot.day(), Weekday::Friday);
/// assert_eq!(slot.from(), 14);
/// assert_eq!(slot.to(), 15);
/// assert!(!slot.expired());
/// ```
#[proc_macro_derive(BitfieldSpecifier, attributes(bits))]
pub fn bitfield_specifier(input: TokenStream) -> TokenStream {
    bitfield_specifier::generate(input.into()).into()
}
