# 0.11.2 (2020-11-07)

- Fixed a bug that all but the first `#[skip(..)]` attribute for a bitfield were ignored despite proper error handling.
  E.g. when flagging a field with `#[skip(getters)] #[skip(setters)]` the `#[skip(setters)]` was ignored.

# 0.11.1 (2020-11-07)

- Allow non-overlapping `#[skip(getters)]` and `#[skip(setters)]` attributes. This might be useful for conditional compilation.
- Fixed a bug with `#[skip]` skipped fields not properly bumping the bit offset
- Fixed a minor bug where duplicate `#[skip]` attributes would yield a confusing error.

# 0.11.0 (2020-11-06)

- Add `bits: int` parameter to the `#[bitfield]` macro which allows to precisely control the resulting bit width
  of the generated bitfield struct. Use it like: `#[bitfield(bits = 5)]`.
- Replace `#[bitfield(specifier = true)]` syntax with `#[bitfield] #[derive(BitfieldSpecifier)]`.
  Semantics of the new syntax is the same as the old.
- It is now possible to flag fields of `#[bitfield]` structs with `#[skip(..)]` in order to skip code generation for them.
  There are possibilities to skip code generation of only setters using `#[skip(setters)]`, only getters `#[skip(getters)]`
  or both. Having no arguments (e.g. just `#[skip]`) defaults to skipping both setters and getters.
  A neat trick is to specify double wildcards as identifiers for skipped fields to avoid having the need to come up with
  an identifier for them: For example: `#[skip]: __: B10`
- Attributes applied to `#[bitfield]` fields are now properly propagated to their generated getters and setters.
  Note thought that it might be confusing that an attribute applied to a struct field is actually applied to a function
  through macro expansion.
- Fixed several bugs and significantly improved error reporting for `#[bits = N]` field attributes for `#[bitfield]` fields.
- Minor fixes and improvements to code generation:
  - The automatically implemented `#[derive(Debug)]` now produces code that the Rust 2015 edition will accept.
  - Some minor macro hygiene improvements.

# 0.10.0 (2020-10-29)

- (Thanks @jam1garner): The `#[bitfield]` macro now looks for `#[derive(Debug)]` annotations and if found will implement
  one that formats the struct with its fields as a user would expect them. Previously having a `#[derive(Debug)]`
  annotation on a `#[bitfield]` struct would simply print the contents of the generated internal byte array.
- Implement `#[bitfield(bytes = N)]` parameter to ensure at compile time that the bitfield struct
  requires `N` bytes of memory.
- Implement `#[bitfield(filled: bool)]` parameter to control whether the bitfield struct ensures that either
  all bits are well defined (`filled = true`) or if there are going to be some bits intentionally undefined (`filled = false`).
  The default is `filled = true`.
- Implement `#[repr(uN)]` for `#[bitfield]` structs where `uN` is one of `u8`, `u16`, `u32`, `u64` or `u128` with which it is
  possible to control whether a bitfield allow conversions between `uN` and the bitfield. Also it ensures at compile time that
  the bitfield requires the exact same amount of bits. This is in conflict with `filled = false`.
- Bitfield structs are no longer implicitly `#[repr(transparent)]`. If a user wants their bitfield struct to remain transparent
  they have to add `#[repr(transparent)]` manually to their struct definition.
- The default behaviour for `#[bitfield(specifier = bool)]` got changed so that it no longer by default allows for
  unfilled (`filled = false`) or undefined bits. Instead users now have to additionally add `filled = false` if they
  explicitly want this behaviour.
- Renamed the generated `as_bytes` method for `#[bitfield]` structs to `into_bytes`. It now takes `self` instead of `&self`
  and returns the byte array by value instead of reference. This change was necessary for working properly with the new
  `#[repr(uN)]` feature.
- Fixed a bug with `#[bitfield(specifier = true)]` bitfields that were not aligned to have a power-of-two bytes.

# 0.9.0 (2020-10-26)

- Add `#[bitfield(specifier = bool)]` parameter with which it now is possible to have bitfield structs automatically also
  implement the `modular_bitfield::Specifier` trait which makes it possible to have bitfields as fields of bitfields.
- No longer generates an `unsafe fn from_bytes_unchecked`. Now generates a safe `fn from_bytes` that is basically identical.
  The difference is that we no longer consider bitfields containing invalid bit patterns as invalid since generated getters
  will protect their access anyways.
- Update crate documentation and README.

# 0.8.0 (2020-10-25)

- The `#[derive(BitfieldSpecifier)]` now allows an amount of variants that is not a power of two via
  the new attribute `#[bits = N]` where `N` is the bit width of the deriving enum. (Thanks @lkolbly)
- Add `Specifier` implementations for `u8`, `u16`, `u32`, `u64` and `u128`.
- The `#[bitfield]` macro now additionally generates getters for expected failures just as it already
  does for the various setters. For a field `a` the new fail-safe getter is called `a_or_err` and returns
  a `Result`.
- Silence repetitive `dead_code` warnings originating from generated `#[bitfield]` getters and setters.
- Improve error span information in a few use cases.
- Cleaned up backend code for `modular_bitfield` crate and its generated code.

# 0.7.0 (2020-10-18)

- Tons of macro generated code hygiene improvements for both `#[bitfield]` and `#[derive(BitfieldSpecifier)]`.
- `#[bitfield]` now produces `with_{}` and `with_{}_checked` methods that take `self` and return `Self`. (Thanks @lkolbly)
    - This is useful for building up your bitfields without the need of a mutable reference.
- `#[bitfield]` generated getters now are the same identifiers as their fields. (Thanks @Qyriad)
    - Previously for a field `a` it generated `fn get_a(..)` but now it generates `fn a(..)` instead.
- `#[bitfield]` now properly inherits visibility modifiers for all fields onto their generated getters and setters. (Thanks @WorldSEnder)
- Remove `TryFrom` implementation from `#[bitfield]` generated structs since it never was really safe.
    - It didn't check for invalid bit patterns and thus could trigger undefined behaviour.
- The `#[bitfield]` generated `new` constructor is now a `const fn`.
- Rename the `#[bitfield]` generated `to_bytes` to `as_bytes`.
    - Also it now returns a reference to an array, e.g. `&[u8; N]` instead of a slice.
- The `#[bitfield]` macro now generates an `unsafe fn from_bytes_unchecked` that allows constructing from raw bytes.
    - This is meant as an `unsafe` replacement for the removed `TryFrom` implementation.
    - We plan to create safer alternatives in future updates.
- Updated all benchmarks to use the `criterion` crate.
- Fixed a parsing bug that `enum` variant names sometimes could clash with Rust keywords. (Thanks @Qyriad)
- Fixed a bug that caused getters to sometimes read with extra invalid bits. (Thanks @crzysdrs)
- Fixed an issue that was caused by single bit manipulations. (Thanks @crzysdrs)
