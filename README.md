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

We tested the following `struct`:

```rust
#[bitfield]
pub struct Generated {
    pub a: B9,  // Spans 2 bytes.
    pub b: B6,  // Within 2nd byte.
    pub c: B13, // Spans 3 bytes.
    pub d: B4,  // Within 4rd byte.
    pub e: B32, // Spans rest 4 bytes.
}
```

**Note:** All benchmarks timing results sum 10 runs each.

### Getter Performance

```
cmp_get_a/generated     time:   [3.0490 ns 3.0628 ns 3.0791 ns]
cmp_get_a/handwritten   time:   [3.0640 ns 3.0782 ns 3.0928 ns]

cmp_get_b/generated     time:   [3.0600 ns 3.0731 ns 3.0871 ns]
cmp_get_b/handwritten   time:   [3.0457 ns 3.0592 ns 3.0744 ns]

cmp_get_c/generated     time:   [3.0762 ns 3.1040 ns 3.1368 ns]
cmp_get_c/handwritten   time:   [3.0638 ns 3.0782 ns 3.0934 ns]

cmp_get_d/generated     time:   [3.0603 ns 3.0729 ns 3.0869 ns]
cmp_get_d/handwritten   time:   [3.0833 ns 3.1358 ns 3.2064 ns]

cmp_get_e/generated     time:   [3.0688 ns 3.0915 ns 3.1203 ns]
cmp_get_e/handwritten   time:   [3.0634 ns 3.0753 ns 3.0877 ns]
```

### Setter Performance

```
cmp_set_a/generated     time:   [15.643 ns 15.707 ns 15.775 ns]
cmp_set_a/handwritten   time:   [15.593 ns 15.661 ns 15.736 ns]

cmp_set_b/generated     time:   [20.334 ns 20.439 ns 20.550 ns]
cmp_set_b/handwritten   time:   [20.262 ns 20.327 ns 20.397 ns]

cmp_set_c/generated     time:   [19.634 ns 19.847 ns 20.111 ns]
cmp_set_c/handwritten   time:   [19.544 ns 19.632 ns 19.729 ns]

cmp_set_d/generated     time:   [20.316 ns 20.376 ns 20.437 ns]
cmp_set_d/handwritten   time:   [20.291 ns 20.371 ns 20.457 ns]

cmp_set_e/generated     time:   [6.1394 ns 6.1640 ns 6.1873 ns]
cmp_set_e/handwritten   time:   [6.1172 ns 6.1459 ns 6.1767 ns]
```

## License

Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this codebase by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.

[procedural-macro-workshop]: https://github.com/dtolnay/proc-macro-workshop/blob/master/README.md
[benchmark-code]: ./benches/get_and_set.rs
