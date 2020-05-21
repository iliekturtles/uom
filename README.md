uom
===
[![Travis](https://travis-ci.org/iliekturtles/uom.svg?branch=master)](https://travis-ci.org/iliekturtles/uom)
[![Coveralls](https://coveralls.io/repos/github/iliekturtles/uom/badge.svg?branch=master)](https://coveralls.io/github/iliekturtles/uom?branch=master)
[![Rustup.rs](https://img.shields.io/badge/rustc-1.31.0%2B-orange.svg)](https://rustup.rs/)
[![Crates.io](https://img.shields.io/crates/v/uom.svg)](https://crates.io/crates/uom)
[![Crates.io](https://img.shields.io/crates/l/uom.svg)](https://crates.io/crates/uom)
[![Documentation](https://img.shields.io/badge/documentation-docs.rs-blue.svg)](https://docs.rs/uom)

Units of measurement is a crate that does automatic type-safe zero-cost
[dimensional analysis][analysis]. You can create your own systems or use the pre-built
[International System of Units][si] (SI) which is based on the
[International System of Quantities][isq] (ISQ) and includes numerous [quantities][quantity]
(length, mass, time, ...) with conversion factors for even more numerous
[measurement units][measurement] (meter, kilometer, foot, mile, ...). No more crashing your
[climate orbiter][orbiter]!

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
uom = "0.28.0"
```

and this to your crate root:

```rust
extern crate uom;
```

The simple example below shows how to use quantities and units as well as how `uom` stops invalid
operations:

```rust
extern crate uom;

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

 * [si.rs](examples/si.rs) -- Shows how to use the pre-built SI system.
 * [base.rs](examples/base.rs) -- Shows how to create a set of `Quantity` type aliases for a
   different set of base units. See the [Design](#design) section for implications of choosing
   different base units.
 * [mks.rs](examples/mks.rs) -- Shows how to create a custom system of quantities.

## Features
`uom` has multiple `Cargo` features for controlling available underlying storage types, the
inclusion of the pre-built [International System of Units][si] (SI), support for [Serde][serde],
and `no_std` functionality. The features are described below. `f32`, `f64`, `std`, and `si` are
enabled by default. Features can be cherry-picked by using the `--no-default-features` and
`--features "..."` flags when compiling `uom` or specifying features in Cargo.toml:

```toml
[dependencies]
uom = {
    version = "0.28.0",
    default-features = false,
    features = [
        "autoconvert", # automatic base unit conversion.
        "usize", "u8", "u16", "u32", "u64", "u128", # Unsigned integer storage types.
        "isize", "i8", "i16", "i32", "i64", "i128", # Signed integer storage types.
        "bigint", "biguint", # Arbitrary width integer storage types.
        "rational", "rational32", "rational64", "bigrational", # Integer ratio storage types.
        "f32", "f64", # Floating point storage types.
        "si", "std", # Built-in SI system and std library support.
        "try-from", # `TryFrom` support between `Time` and `Duration`. Requires `rustc` 1.34.0.
        "use_serde", # Serde support.
    ]
}
```

 * `autoconvert` -- Feature to enable automatic conversion between base units in binary operators.
   Disabling the feature only allows for quantities with the same base units to directly interact.
   The feature exists to account for compiler limitations where zero-cost code is not generated for
   non-floating point underlying storage types.
 * `usize`, `u8`, `u16`, `u32`, `u64`, `u128`, `isize`, `i8`, `i16`, `i32`, `i64`, `i128`, `bigint`,
   `biguint`, `rational`, `rational32`, `rational64`, `bigrational`, `f32`, `f64` -- Features to
   enable underlying storage types. At least one of these features must be enabled. `f32` and `f64`
   are enabled by default. See the [Design](#design) section for implications of choosing different
   underlying storage types.
 * `si` -- Feature to include the pre-built [International System of Units][si] (SI). Enabled by
   default.
 * `std` -- Feature to compile with standard library support. Disabling this feature compiles `uom`
   with `no_std`. Enabled by default.
 * `try-from` -- Feature to enable `TryFrom` support between `Time` and `Duration`. Requires `rustc`
   1.34.0.
 * `use_serde` -- Feature to enable support for serialization and deserialization of quantities
   with the [Serde][serde] crate. Disabled by default.

[si]: http://jcgm.bipm.org/vim/en/1.16.html
[serde]: https://serde.rs/

## Design
Rather than working with [measurement units](http://jcgm.bipm.org/vim/en/1.9.html) (meter,
kilometer, foot, mile, ...) `uom` works with [quantities](http://jcgm.bipm.org/vim/en/1.1.html)
(length, mass, time, ...). This simplifies usage because units are only involved at interface
boundaries: the rest of your code only needs to be concerned about the quantities involved. This
also makes operations on quantities (+, -, \*, /, ...) have zero runtime cost over using the raw
storage type (e.g. `f32`).

`uom` normalizes values to the [base unit](http://jcgm.bipm.org/vim/en/1.10.html) for the quantity.
Alternative base units can be used by executing the macro defined for the system of quantities
(`ISQ!` for the SI). `uom` supports `usize`, `u8`, `u16`, `u32`, `u64`, `u128`, `isize`, `i8`,
`i16`, `i32`, `i64`, `i128`, `bigint`, `biguint`, `rational`, `rational32`, `rational64`,
`bigrational`, `f32`, and `f64` as the underlying storage type.

A consequence of normalizing values to the base unit is that some values may not be able to be
represented or can't be precisely represented for floating point and rational underlying storage
types. For example if the base unit of `length` is `meter` and the underlying storage type is `i32`
then values like `1 centimeter` or `1.1 meter` cannot be represented. `1 centimeter` is normalized
to `0.01 meter` which can't be stored in an `i32`. `uom` only allows units to be used safely. Users
of this library will still need to be aware of implementation details of the underlying storage type
including limits and precision.

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
