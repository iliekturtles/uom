uom
===
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

[![Travis](https://travis-ci.org/iliekturtles/uom.svg?branch=master)](https://travis-ci.org/iliekturtles/uom)
[![AppVeyor](https://ci.appveyor.com/api/projects/status/github/iliekturtles/uom?svg=true&branch=master)](https://ci.appveyor.com/project/iliekturtles/uom)
[![Coveralls](https://coveralls.io/repos/github/iliekturtles/uom/badge.svg?branch=master)](https://coveralls.io/github/iliekturtles/uom?branch=master)
[![Crates.io](https://img.shields.io/crates/v/uom.svg)](https://crates.io/crates/uom)
[![Crates.io](https://img.shields.io/crates/l/uom.svg)](https://crates.io/crates/uom)

[Documentation](https://docs.rs/uom)

## Usage
`uom` requires `rustc` 1.15.0 or later. Add this to your `Cargo.toml`:

```toml
[dependencies]
uom = "0.11.0"
```

and this to your crate root:

```rust
extern crate uom;
```

The simple example below shows how to use quantities and units as well as how `uom` stops invalid
operations.

```rust
extern crate uom;

use uom::si::f32::*;
use uom::si::length::kilometer;
use uom::si::time::second;

fn main() {
	let length = Length::new(5.0, kilometer);
	let time = Time::new(15.0, second);
	let velocity = length / time;
	//let error = length + time; // error[E0308]: mismatched types
}
```

See the [examples](examples) directory for more advanced usage:

 * [si.rs](examples/si.rs) -- Example showing how to use the pre-built SI system.
 * [base.rs](examples/base.rs) -- Example showing how to create a set of `Quantity` type aliases
   for a different set of base units.
 * [mks.rs](examples/mks.rs) -- Example showing how to create a custom system of quantities.

## Design
Rather than working with [measurement units](http://jcgm.bipm.org/vim/en/1.9.html) (meter,
kilometer, foot, mile, ...) `uom` works with [quantities](http://jcgm.bipm.org/vim/en/1.1.html)
(length, mass, time, ...). This simplifies usage because units are only involved at interface
boundaries: the rest of your code only needs to be concerned about the quantities involved. This
also makes operations on quantities (+, -, \*, /, ...) have zero runtime cost<sup>1</sup> over
using the raw storage type (e.g. `f32`).

`uom` normalizes values to the [base unit](http://jcgm.bipm.org/vim/en/1.10.html) for the quantity.
Alternative base units can be used by executing the macro defined for the system of quantities
(`ISQ!` for the SI). `uom` supports both `f32` and `f64` as the underlying storage type.

 1. Once codegen bug [#38269](https://github.com/rust-lang/rust/issues/38269) is resolved.

## License
Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
