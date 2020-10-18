# 0.8.0 (WIP)

- Add `Specifier` implementations for `u8`, `u16`, `u32`, `u64` and `u128`.

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
    - Also it now returns a refernce to an array, e.g. `&[u8; N]` instead of a slice.
- The `#[bitfield]` macro now generates an `unsafe fn from_bytes_unchecked` that allows constructing from raw bytes.
    - This is meant as an `unsafe` replacement for the removed `TryFrom` implementation.
    - We plan to create safer alternatives in future updates.
- Updated all benchmarks to use the `criterion` crate.
- Fixed a parsing bug that `enum` variant names sometimes could clash with Rust keywords. (Thanks @Qyriad)
- Fixed a bug that caused getters to sometimes read with extra invalid bits. (Thanks @crzysdrs)
- Fixed an issue that was caused by single bit manipulations. (Thanks @crzysdrs)
