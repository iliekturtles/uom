uom
===
[![Travis](https://travis-ci.org/iliekturtles/uom.svg?branch=master)](https://travis-ci.org/iliekturtles/uom)
[![Coveralls](https://coveralls.io/repos/github/iliekturtles/uom/badge.svg?branch=master)](https://coveralls.io/github/iliekturtles/uom?branch=master)
[![Rustup.rs](https://img.shields.io/badge/rustc-1.31.0%2B-orange.svg)](https://rustup.rs/)
[![Crates.io](https://img.shields.io/crates/v/uom.svg)](https://crates.io/crates/uom)
[![Crates.io](https://img.shields.io/crates/l/uom.svg)](https://crates.io/crates/uom)
[![Documentation](https://img.shields.io/badge/documentation-docs.rs-blue.svg)](https://docs.rs/uom)

`uom`, Units of measurement, is a crate that does automatic type-safe zero-cost [dimensional
analysis][analysis]. You can create your own systems or use the pre-built [International System of
Quantities][isq] (ISQ) which includes numerous [quantities][quantity] (length, mass, time, ...) with
conversion factors for even more numerous [measurement units][measurement] (meter, kilometer, foot,
mile, ...). No more crashing your [climate orbiter][orbiter]!

[analysis]: https://en.wikipedia.org/wiki/Dimensional_analysis
[si]: http://jcgm.bipm.org/vim/en/1.16.html
[isq]: http://jcgm.bipm.org/vim/en/1.6.html
[quantity]: http://jcgm.bipm.org/vim/en/1.1.html
[measurement]: http://jcgm.bipm.org/vim/en/1.9.html
[orbiter]: https://en.wikipedia.org/wiki/Mars_Climate_Orbiter

## Usage
`uom` requires `rustc` 1.31.0 or later. Add this to your `Cargo.toml`:

```toml
[dependencies]
uom = "0.25.0"
```

The simple example below shows how to use quantities and units as well as how `uom` stops invalid
operations:

```rust
use uom::si::f32::*;
use uom::si::length::kilometer;
use uom::si::time::second;

fn main() {
    let length = Length::new::<kilometer>(5.0);
    let time = Time::new::<second>(15.0);
    let velocity/*: Velocity*/ = length / time;
    let _acceleration = calc_acceleration(velocity, time);
    //let error = length + time; // error[E0308]: mismatched types
}

fn calc_acceleration(velocity: Velocity, time: Time) -> Acceleration {
    velocity / time
}
```

See the [examples](examples) directory for more advanced usage:

## Features
`uom` has multiple `Cargo` features for controlling available underlying storage types, the
inclusion of the pre-built [International System of Quantities][isq] (ISQ), support for
[Serde][serde], and `no_std` functionality. The features are described below. `f32`, `f64`, `std`,
and `isq` are enabled by default. Features can be cherry-picked by using the `--no-default-features`
and `--features "..."` flags when compiling `uom` or specifying features in Cargo.toml:

```toml
[dependencies]
uom = {
    version = "0.25.0",
    default-features = false,
    features = [
        "autoconvert", # automatic base unit conversion.
        "usize", "u8", "u16", "u32", "u64", "u128", # Unsigned integer storage types.
        "isize", "i8", "i16", "i32", "i64", "i128", # Signed integer storage types.
        "bigint", "biguint", # Arbitrary width integer storage types.
        "rational", "rational32", "rational64", "bigrational", # Integer ratio storage types.
        "f32", "f64", # Floating point storage types.
        "isq", "std", # Built-in ISQ and std library support.
        "use_serde", # Serde support.
    ]
}
```

 * `usize`, `u8`, `u16`, `u32`, `u64`, `u128`, `isize`, `i8`, `i16`, `i32`, `i64`, `i128`, `bigint`,
   `biguint`, `rational`, `rational32`, `rational64`, `bigrational`, `f32`, `f64` -- Features to
   enable underlying storage types. At least one of these features must be enabled. `f32` and `f64`
   are enabled by default. See the [Design](#design) section for implications of choosing different
   underlying storage types.
 * `isq` -- Feature to include the pre-built [International System of Quantities][isq] (ISQ).
   Enabled by default.
 * `std` -- Feature to compile with standard library support. Disabling this feature compiles `uom`
   with `no_std`. Enabled by default.
 * `use_serde` -- Feature to enable support for serialization and deserialization of quantities
   with the [Serde][serde] crate. Disabled by default.

[isq]: http://jcgm.bipm.org/vim/en/1.6.html
[serde]: https://serde.rs/

## Design

## Contributing
Contributions are welcome from everyone. Submit a pull request, an issue, or just add comments to an
existing item. The [International Bureau of Weights and Measures][BIPM] is an international
standards organization that publishes the [SI Brochure][brochure]. This document defines the [SI]
and can be used as a comprehensive reference for changes to `uom`. Conversion factors for non-SI
units can be found in NIST [Special Publication 811][nist811].

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in
the work by you, as defined in the Apache-2.0 license, shall be dual licensed as below, without any
additional terms or conditions.

### License
Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   <http://www.apache.org/licenses/LICENSE-2.0>)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

[BIPM]: http://www.bipm.org/en/about-us/
[brochure]: http://www.bipm.org/en/publications/si-brochure/
[si]: http://jcgm.bipm.org/vim/en/1.16.html
[nist811]: https://www.nist.gov/pml/nist-guide-si-appendix-b9-factors-units-listed-kind-quantity-or-field-science
