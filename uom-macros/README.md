uom-macros
===
[![Travis](https://travis-ci.org/iliekturtles/uom.svg?branch=master)](https://travis-ci.org/iliekturtles/uom)
[![Coveralls](https://coveralls.io/repos/github/iliekturtles/uom/badge.svg?branch=master)](https://coveralls.io/github/iliekturtles/uom?branch=master)
[![Rustup.rs](https://img.shields.io/badge/rustc-1.68.0%2B-orange.svg)](https://rustup.rs/)
[![Crates.io](https://img.shields.io/crates/v/uom-macros.svg)](https://crates.io/crates/uom-macros)
[![Crates.io](https://img.shields.io/crates/l/uom-macros.svg)](https://crates.io/crates/uom-macros)
[![Documentation](https://img.shields.io/badge/documentation-docs.rs-blue.svg)](https://docs.rs/uom-macros)

⚠ This crate is currently a placeholder for future development efforts. ⚠

`uom-macros` provides procedural macro support for `uom`. Two function-style macros are available.
`system!`, to define a system of quantities and a related system of units, and `quantity!`, to
define quantities within a system.  See the [`uom`](https://crates.io/crates/uom) crate for full
details.

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
   <https://www.apache.org/licenses/LICENSE-2.0>)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or <https://opensource.org/licenses/MIT>)

at your option.

[BIPM]: https://www.bipm.org/en/about-us/
[brochure]: https://www.bipm.org/en/publications/si-brochure/
[si]: https://jcgm.bipm.org/vim/en/1.16.html
[nist811]: https://www.nist.gov/pml/special-publication-811/nist-guide-si-appendix-b-conversion-factors/nist-guide-si-appendix-b9
