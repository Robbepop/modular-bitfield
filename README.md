# Modular Bitfields for Rust

|   Continuous Integration  |  Documentation    |       Crates.io      |       LoC        |
|:-------------------------:|:-----------------:|:--------------------:|:----------------:|
| [![GHActions][C1]][C2]    | [![docs][A1]][A2] | [![crates][B1]][B2]  | [![loc][D1]][D2] |

[A1]: https://docs.rs/modular-bitfield/badge.svg
[A2]: https://docs.rs/modular-bitfield
[B1]: https://img.shields.io/crates/v/modular_bitfield.svg
[B2]: https://crates.io/crates/modular_bitfield
[C1]: https://github.com/Robbepop/modular-bitfield/workflows/Rust%20-%20Continuous%20Integration/badge.svg?branch=master&event=push
[C2]: https://github.com/Robbepop/modular-bitfield/actions?query=workflow%3A%22Rust+-+Continuous+Integration%22+branch%3Amaster+event%3Apush
[D1]: https://tokei.rs/b1/github/Robbepop/modular-bitfield?category=code
[D2]: https://github.com/Aaronepower/tokei#badges

- `no_std`: Supports embedded development without `std` library.
- This crate uses 100% safe Rust code.

## Description

Allows to have bitfield structs and enums as bitfield specifiers that work very similar to C and C++ bitfields.

## Advantages

- **Safety:** Macro embraced enums and structs are checked for valid structure during compilation time.
- **Speed:** Generated code is as fast as handwritten code. (See benchmarks below.)
- **Modularity:** Enums can be used modular within bitfield structs.

## Attribution

Implements the `#[bitfield]` macros introduced and specified in David Tolnay's [procedural macro workshop][procedural-macro-workshop].

Thanks go to David Tolnay for designing the specification for the macros implemented in this crate.

## Example

```rust
use modular_bitfield::prelude::*;

// Works with aliases - just for the showcase.
type Vitamin = B12;

/// Bitfield struct with 32 bits in total.
#[bitfield]
#[derive(Debug, PartialEq, Eq)]
pub struct Example {
    a: bool,         // Uses 1 bit
    b: B9,           // Uses 9 bits
    c: Vitamin,      // Uses 12 bits, works with aliases.
    #[bits = 3]      // Optional, asserts at compiletime that `DeliveryMode` uses 3 bits.
    d: DeliveryMode, // Uses 3 bits
    e: B7,           // Uses 7 bits
}

/// Enums that derive from `BitfieldSpecifier`
/// can also be used within bitfield structs
/// as shown above.
#[derive(BitfieldSpecifier, Debug, PartialEq)]
pub enum DeliveryMode {
    Fixed = 1,
    Lowest,
    SMI,
    RemoteRead,
    NMI,
    Init = 0,
    Startup = 6,
    External,
}

fn main() {
    let mut example = Example::new();

    // Assert that everything is inizialized to 0.
    assert_eq!(example.a(), false);
    assert_eq!(example.b(), 0);
    assert_eq!(example.c(), 0);
    assert_eq!(example.d(), DeliveryMode::Init);
    assert_eq!(example.e(), 0);

    // Modify the bitfields.
    example.set_a(true);
    example.set_b(0b0001_1111_1111_u16);  // Uses `u16`
    example.set_c(42_u16);                // Uses `u16`
    example.set_d(DeliveryMode::Startup);
    example.set_e(1);                     // Uses `u8`

    // Assert the previous modifications.
    assert_eq!(example.a(), true);
    assert_eq!(example.b(), 0b0001_1111_1111_u16);
    assert_eq!(example.c(), 42);
    assert_eq!(example.d(), DeliveryMode::Startup);
    assert_eq!(example.e(), 1_u8);

    // Safe API allows for better testing
    assert_eq!(example.set_e_checked(200), Err(Error::OutOfBounds));

    // Can convert from and to bytes.
    assert_eq!(example.to_bytes(), &[255, 171, 128, 3]);
    use std::convert::TryFrom as _;
    let copy = Example::from_bytes(example.to_bytes());
    assert_eq!(example, copy);
}
```

## Benchmarks

Below are some benchmarks between the [hand-written code][benchmark-code] and the macro-generated code for some example getters and setters that cover a decent variety of use cases.

We can conclude that the macro-generated code is as fast as hand-written code would be. Please file a PR if you see a way to improve either side.

- `cargo bench` to run the benchmarks
- `cargo test --benches` to run the benchmark tests

[Click here to view all benchmark results.](https://gist.github.com/Robbepop/bcff4fe149e0e622b752f0eb07b31880)

### Summary

The `modular_bitfield` crate generates bitfields that are ...

- just as efficient as the handwritten alternatives.
- equally efficient or more efficient than the alternative [bitfield] crate.

[bitfield]: https://crates.io/crates/bitfield

### Showcase: Generated vs Handwritten

We tested the following `#[bitfield]` `struct`:

```rust
#[bitfield]
pub struct Generated {
    pub a: B9,  // Spans 2 bytes.
    pub b: B6,  // Within 2nd byte.
    pub c: B13, // Spans 3 bytes.
    pub d: B1,  // Within 4rd byte.
    pub e: B3,  // Within 4rd byte.
    pub f: B32, // Spans rest 4 bytes.
}
```

**Note:** All benchmarks timing results sum 10 runs each.

### Getter Performance

```
get_a/generated     time:   [3.0990 ns 3.1119 ns 3.1263 ns]
get_a/handwritten   time:   [3.1072 ns 3.1189 ns 3.1318 ns]

get_b/generated     time:   [3.0859 ns 3.0993 ns 3.1140 ns]
get_b/handwritten   time:   [3.1062 ns 3.1154 ns 3.1244 ns]

get_c/generated     time:   [3.0892 ns 3.1140 ns 3.1491 ns]
get_c/handwritten   time:   [3.1031 ns 3.1144 ns 3.1266 ns]

get_d/generated     time:   [3.0937 ns 3.1055 ns 3.1182 ns]
get_d/handwritten   time:   [3.1109 ns 3.1258 ns 3.1422 ns]

get_e/generated     time:   [3.1009 ns 3.1139 ns 3.1293 ns]
get_e/handwritten   time:   [3.1217 ns 3.1366 ns 3.1534 ns]

get_f/generated     time:   [3.1064 ns 3.1164 ns 3.1269 ns]
get_f/handwritten   time:   [3.1067 ns 3.1221 ns 3.1404 ns]
```

### Setter Performance

```
set_a/generated     time:   [15.784 ns 15.855 ns 15.932 ns]
set_a/handwritten   time:   [15.841 ns 15.907 ns 15.980 ns]

set_b/generated     time:   [20.496 ns 20.567 ns 20.643 ns]
set_b/handwritten   time:   [20.319 ns 20.384 ns 20.454 ns]

set_c/generated     time:   [19.155 ns 19.362 ns 19.592 ns]
set_c/handwritten   time:   [19.265 ns 19.383 ns 19.523 ns]

set_d/generated     time:   [12.325 ns 12.376 ns 12.429 ns]
set_d/handwritten   time:   [12.416 ns 12.472 ns 12.541 ns]

set_e/generated     time:   [20.460 ns 20.528 ns 20.601 ns]
set_e/handwritten   time:   [20.473 ns 20.534 ns 20.601 ns]

set_f/generated     time:   [6.1466 ns 6.1769 ns 6.2127 ns]
set_f/handwritten   time:   [6.1467 ns 6.1962 ns 6.2670 ns]
```

## License

Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this codebase by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.

[procedural-macro-workshop]: https://github.com/dtolnay/proc-macro-workshop/blob/master/README.md
[benchmark-code]: ./benches/get_and_set.rs
