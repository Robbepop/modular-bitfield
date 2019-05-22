# Modular Bitfields for Rust

This crate implements the `#[bitfield]` macros introduced and specified in David Tolnay's [procedural macro workshop][procedural-macro-workshop].

Check out his workshop and learn from it!

Thanks go to David Tolnay for designing the specification for the macros implemented in `modular_bitfield`.

### Showcase

```rust
use modular_bitfield::prelude::*;

// Works with aliases - just for the showcase.
type Vitamin = B12;

#[bitfield]
pub struct Example {
    a: bool,         // Uses 1 bit
    b: B7,           // Uses 9 bits
    c: Vitamin,      // Uses 11 bits, works with aliases.
    #[bits = 3]      // Optional, asserts at compiletime that `DeliveryMode` uses 3 bits.
    d: DeliveryMode, // Uses 3 bits
}

/// Enums that derive from `BitfieldSpecifier`
/// can also be used within bitfield structs
/// as shown above.
#[derive(BitfieldSpecifier, Debug, PartialEq)]
pub enum DeliveryMode {
    Fixed,
    Lowest,
    SMI,
    RemoteRead,
    NMI,
    Init = 0,
    Startup,
    External,
}

fn it_works() {
    let mut example = Example::new();

    // Assert that everything is inizialized to 0.
    assert_eq!(example.get_a(), false);
    assert_eq!(example.get_b(), 0);
    assert_eq!(example.get_c(), 0);
    assert_eq!(example.get_d(), DeliveryMode::Init);

    // Modify the bitfields.
    example.set_a(true);
    example.set_b(0b111_1111_u8); // Uses `u8`
    example.set_c(42_u16);        // Uses `u16`
    example.set_d(DeliveryMode::Startup);

    // Assert the previous modifications.
    assert_eq!(example.get_a(), true);
    assert_eq!(example.get_b(), 0b111_1111_u8);
    assert_eq!(example.get_c(), 42_u16);
    assert_eq!(example.get_d(), DeliveryMode::Startup);
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

### Future Plans

- Add `no_std` support behind crate feature.
- Add `u128` support behind crate feature.
    - No default because it might explode compilation times.
- Implement non-powers-of-two enums. (Specs from workshop.)
- Implement codegen for tuple structs
    - Example `struct Example(B1, B7, B8)`
      with getters
        - `get_0` and `set_0` for `B1`
        - `get_1` and `set_1` for `B7`
        - `get_2` and `set_2` for `B8`
- Implement safe `try_set_*` for bitfield structs as checked setters that won't panic upon out of bounds input.
- Implement unsafe `set_*_unchecked` for bitfield struct to allow users to avoid bounds checking.

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
