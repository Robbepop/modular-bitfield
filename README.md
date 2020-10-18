# Modular Bitfields for Rust

|   Continuous Integration  |  Documentation   |       Crates.io      |
|:-------------------------:|:----------------:|:--------------------:|
| [![GHActions][C1]][C2]    | [![docs][A1]][A2] | [![crates][B1]][B2]  |

[A1]:  https://docs.rs/string-interner/badge.svg
[A2]: https://docs.rs/string-interner
[B1]: https://img.shields.io/crates/v/string-interner.svg
[B2]: https://crates.io/crates/string-interner
[C1]: https://github.com/Robbepop/modular-bitfield/workflows/Rust%20-%20Continuous%20Integration/badge.svg
[C2]: https://github.com/Robbepop/modular-bitfield/actions?query=workflow%3A%22Rust+-+Continuous+Integration%22

This crate implements the `#[bitfield]` macros introduced and specified in David Tolnay's [procedural macro workshop][procedural-macro-workshop].

Check out his workshop and learn from it!

Thanks go to David Tolnay for designing the specification for the macros implemented in `modular_bitfield`.

### Showcase

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
    let copy = Example::try_from(example.to_bytes()).unwrap();
    assert_eq!(example, copy);
}
```

### Advantages

- **Safety:** Macro embraced enums and structs are checked for valid structure during compilation time.
- **Speed:** Generated code is as fast as handwritten code.
- **Modularity:** Enums can be used modular within bitfield structs.

### Benchmarks

Below are some benchmarks between the [hand-written code][benchmark-code] and the macro-generated code for some example getters and setters that cover a decent variety of use cases.

We can conclude that the macro-generated code is as fast as hand-written code would be. Please file a PR if you see a way to improve either side.

- `cargo bench` to run the benchmarks
- `cargo test --benches` to run the benchmark tests

```
generated::get_a   ... bench:          99 ns/iter (+/- 4)
generated::get_b   ... bench:          98 ns/iter (+/- 4)
generated::get_c   ... bench:          98 ns/iter (+/- 14)
generated::get_d   ... bench:          97 ns/iter (+/- 2)
generated::get_e   ... bench:          99 ns/iter (+/- 2)
generated::set_a   ... bench:         481 ns/iter (+/- 19)
generated::set_b   ... bench:         627 ns/iter (+/- 37)
generated::set_c   ... bench:         507 ns/iter (+/- 35)
generated::set_d   ... bench:         622 ns/iter (+/- 24)
generated::set_e   ... bench:         459 ns/iter (+/- 16)

handwritten::get_a ... bench:          99 ns/iter (+/- 3)
handwritten::get_b ... bench:         102 ns/iter (+/- 14)
handwritten::get_c ... bench:         102 ns/iter (+/- 8)
handwritten::get_d ... bench:         100 ns/iter (+/- 20)
handwritten::get_e ... bench:          98 ns/iter (+/- 6)
handwritten::set_a ... bench:         582 ns/iter (+/- 20)
handwritten::set_b ... bench:         614 ns/iter (+/- 35)
handwritten::set_c ... bench:         533 ns/iter (+/- 18)
handwritten::set_d ... bench:         606 ns/iter (+/- 21)
handwritten::set_e ... bench:         456 ns/iter (+/- 21)
```

### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this codebase by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.
</sub>

[procedural-macro-workshop]: https://github.com/dtolnay/proc-macro-workshop/blob/master/README.md
[benchmark-code]: ./benches/get_and_set.rs
