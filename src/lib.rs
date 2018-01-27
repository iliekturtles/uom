//! Units of measurement is a crate that does automatic type-safe zero-cost
//! [dimensional analysis][analysis]. You can create your own systems or use the pre-built
//! [International System of Units][si] (SI) which is based on the
//! [International System of Quantities][isq] (ISQ) and includes numerous [quantities][quantity]
//! (length, mass, time, ...) with conversion factors for even more numerous
//! [measurement units][measurement] (meter, kilometer, foot, mile, ...). No more crashing your
//! [climate orbiter][orbiter]!
//!
//! [analysis]: https://en.wikipedia.org/wiki/Dimensional_analysis
//! [si]: http://jcgm.bipm.org/vim/en/1.16.html
//! [isq]: http://jcgm.bipm.org/vim/en/1.6.html
//! [quantity]: http://jcgm.bipm.org/vim/en/1.1.html
//! [measurement]: http://jcgm.bipm.org/vim/en/1.9.html
//! [orbiter]: https://en.wikipedia.org/wiki/Mars_Climate_Orbiter
//!
//! ## Usage
//! `uom` requires `rustc` 1.20.0 or later. Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! uom = "0.17.0"
//! ```
//!
//! and this to your crate root:
//!
//! ```rust
//! extern crate uom;
//! ```
//!
//! The simple example below shows how to use quantities and units as well as how `uom` stops
//! invalid operations:
//!
#![cfg_attr(all(feature = "si", feature = "f32"), doc = " ```rust")]
#![cfg_attr(not(all(feature = "si", feature = "f32")), doc = " ```rust,ignore")]
//! extern crate uom;
//!
//! use uom::si::f32::*;
//! use uom::si::length::kilometer;
//! use uom::si::time::second;
//!
//! fn main() {
//!     let length = Length::new::<kilometer>(5.0);
//!     let time = Time::new::<second>(15.0);
//!     let _velocity = length / time;
//!     //let error = length + time; // error[E0308]: mismatched types
//! }
//! ```
//!
//! See examples provided with the source for more advanced usage including how to create `Quantity`
//! type aliases for a different set of base units and how to create an entirely new system of
//! quantities.
//!
//! ## Features
//! `uom` has multiple `Cargo` features for controlling available underlying storage types, the
//! inclusion of the pre-built [International System of Units][si] (SI), support for
//! [Serde][serde], and `no_std` functionality. The features are described below. `f32`, `f64`,
//! `std`, and `si` are enabled by default. Features can be cherry-picked by using the
//! `--no-default-features` and `--features "..."` flags when compiling `uom` or specifying
//! features in Cargo.toml:
//!
//! ```toml
//! [dependencies]
//! uom = {
//!     version = "0.17.0",
//!     default-features = false,
//!     features = [
//!         "autoconvert", # automatic base unit conversion.
//!         "usize", "u8", "u16", "u32", "u64", # Unsigned integer storage types.
//!         "isize", "i8", "i16", "i32", "i64", # Signed interger storage types.
//!         "bigint", "biguint", # Arbitrary width integer storage types.
//!         "rational", "rational32", "rational64", "bigrational", # Integer ratio storage types.
//!         "f32", "f64", # Floating point storage types.
//!         "si", "std", # Built-in SI system and std library support.
//!         "use_serde", # Serde support.
//!     ]
//! }
//! ```
//!
//!  * `autoconvert` -- Feature to enable automatic conversion between base units in binary
//!    operators. Disabling the feature only allows for quantities with the same base units to
//!    directly interact. The feature exists to account for compiler limitations where zero-cost
//!    code is not generated for non-floating point underlying storage types.
//!  * `usize`, `u8`, `u16`, `u32`, `u64`, `isize`, `i8`, `i16`, `i32`, `i64`, `bigint`, `biguint`,
//!    `rational`, `rational32`, `rational64`, `bigrational`, `f32`, `f64` -- Features to enable
//!    underlying storage types. At least one of these features must be enabled. `f32` and `f64` are
//!    enabled by default. See the [Design](#design) section for implications of choosing different
//!    underlying storage types.
//!  * `si` -- Feature to include the pre-built [International System of Units][si] (SI). Enabled by
//!    default.
//!  * `std` -- Feature to compile with standard library support. Disabling this feature compiles
//!    `uom` with `no_std`. Enabled by default.
//!  * `use_serde` -- Feature to enable support for serialization and deserialization of quantities
//!    with the [Serde][serde] crate. Disabled by default.
//!
//! [si]: http://jcgm.bipm.org/vim/en/1.16.html
//! [serde]: https://serde.rs/
//!
//! ## Design
//! Rather than working with [measurement units](http://jcgm.bipm.org/vim/en/1.9.html) (meter,
//! kilometer, foot, mile, ...) `uom` works with [quantities](http://jcgm.bipm.org/vim/en/1.1.html)
//! (length, mass, time, ...). This simplifies usage because units are only involved at interface
//! boundaries: the rest of your code only needs to be concerned about the quantities involved. This
//! also makes operations on quantities (+, -, \*, /, ...) have zero runtime cost<sup>1</sup> over
//! using the raw storage type (e.g. `f32`).
//!
//! `uom` normalizes values to the [base unit](http://jcgm.bipm.org/vim/en/1.10.html) for the
//! quantity. Alternative base units can be used by executing the macro defined for the system of
//! quantities (`ISQ!` for the SI). `uom` supports `usize`, `u8`, `u16`, `u32`, `u64`, `isize`,
//! `i8`, `i16`, `i32`, `i64`, `bigint`, `biguint`, `rational`, `rational32`, `rational64`,
//! `bigrational`, `f32`, and `f64` as the underlying storage type.
//!
//! A consequence of normalizing values to the base unit is that some values may not be able to be
//! represented or can't be precisely represented for floating point and rational underlying
//! storage types. For example if the base unit of `length` is `meter` and the underlying storage
//! type is `i32` then values like `1 centimeter` or `1.1 meter` cannot be represented. `1
//! centimeter` is normalized to `0.01 meter` which can't be stored in an `i32`. `uom` only allows
//! units to be used safely. Users of this library will still need to be aware of implementation
//! details of the underlying storage type including limits and precision.
//!
//!  1. Once codegen bug [#38269](https://github.com/rust-lang/rust/issues/38269) is resolved.
//!
//! ## Contributing
//! Contributions are welcome from everyone. Submit a pull request, an issue, or just add comments
//! to an existing item. The [International Bureau of Weights and Measures][BIPM] is an
//! international standards organization that publishes the [SI Brochure][brochure]. This document
//! defines the [SI] and can be used as a comprehensive reference for changes to `uom`. Conversion
//! factors for non-SI units can be found in NIST [Special Publication 811][nist811].
//!
//! Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in
//! the work by you, as defined in the Apache-2.0 license, shall be dual licensed as below, without
//! any additional terms or conditions.
//!
//! ### License
//! Licensed under either of
//!
//!  * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
//!    <http://www.apache.org/licenses/LICENSE-2.0>)
//!  * MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)
//! 
//! at your option.
//!
//! [BIPM]: http://www.bipm.org/en/about-us/
//! [brochure]: http://www.bipm.org/en/publications/si-brochure/
//! [si]: http://jcgm.bipm.org/vim/en/1.16.html
//! [nist811]: https://www.nist.gov/pml/nist-guide-si-appendix-b9-factors-units-listed-kind-quantity-or-field-science

// Compile with `no_std` when the `std` feature is not specified.
#![cfg_attr(not(feature = "std"), no_std)]

// Default rustc lints.
#![warn(
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results)]

// Clippy lints.
#![cfg_attr(feature = "cargo-clippy", allow(inline_always))]

// Fail to compile if no underlying storage type features are specified.
#[cfg(not(any(
    feature = "usize", feature = "u8", feature = "u16", feature = "u32", feature = "u64",
    feature = "isize", feature = "i8", feature = "i16", feature = "i32", feature = "i64",
    feature = "bigint", feature = "biguint",
    feature = "rational", feature = "rational32", feature = "rational64", feature = "bigrational",
    feature = "f32", feature = "f64", )))]
compile_error!("A least one underlying storage type must be enabled. See the features section of \
    uom documentation for available underlying storage type options.");

#[doc(hidden)]
pub extern crate num;

#[doc(hidden)]
#[cfg(feature = "serde")]
pub extern crate serde;

#[doc(hidden)]
pub extern crate typenum;

#[cfg(all(test, any(feature = "f32", feature = "f64")))]
#[macro_use]
extern crate approx;
#[cfg(test)]
#[macro_use]
extern crate quickcheck;
#[cfg(all(test, feature = "serde"))]
extern crate serde_json;
#[cfg(test)]
#[macro_use]
extern crate static_assertions;

// Conditionally import `core` or `std` based on feature selection.
#[doc(hidden)]
pub mod lib {
    #[cfg(not(feature = "std"))]
    pub use core::*;
    #[cfg(feature = "std")]
    pub use std::*;

    // Re-export `ops` module along with `typenum::ops` to provide all types in a single mod. This
    // allows the `system!` macro to reference all operations by the absolute path. Macro paths and
    // idents can't easily be combined without a `use` statement that pollutes the macro execution
    // location's namespace.
    pub mod ops {
        #[cfg(not(feature = "std"))]
        pub use core::ops::*;
        #[cfg(feature = "std")]
        pub use std::ops::*;
        pub use typenum::type_operators::*;
    }
}

#[macro_use]
mod storage_types;

#[macro_use]
mod system;

#[macro_use]
mod quantity;

#[cfg(feature = "si")]
#[macro_use]
pub mod si;

#[cfg(test)]
mod tests;

/// Trait to identify [units][units] which have a [conversion factor][factor].
///
/// [units]: http://jcgm.bipm.org/vim/en/1.13.html
/// [factor]: https://jcgm.bipm.org/vim/en/1.24.html
pub trait Conversion<V> {
    /// Conversion factor type specific to the underlying storage type.
    type T: ConversionFactor<V>;

    /// Static [conversion factor][factor] for the given unit to the base unit for the quantity.
    ///
    /// Default implementation returns `Self::T::one()`.
    ///
    /// [factor]: https://jcgm.bipm.org/vim/en/1.24.html
    #[inline(always)]
    fn conversion() -> Self::T {
        <Self::T as num::One>::one()
    }

    /// Instance [conversion factor][factor].
    ///
    /// Default implementation returns the static conversion `Self::conversion()`.
    ///
    /// [factor]: https://jcgm.bipm.org/vim/en/1.24.html
    #[inline(always)]
    fn into_conversion(&self) -> Self::T
    where
        Self: Sized,
    {
        Self::conversion()
    }
}

/// Trait representing a [conversion factor][factor].
///
/// [factor]: https://jcgm.bipm.org/vim/en/1.24.html
pub trait ConversionFactor<V>
    : lib::ops::Div<Self, Output = Self>
    + lib::ops::Mul<Self, Output = Self>
    + num::One
{
    /// Raises a `ConversionFactor<V>` to an integer power.
    fn powi(self, e: i32) -> Self;

    /// Converts a `ConversionFactor<V>` into its underlying type.
    fn value(self) -> V;
}

storage_types! {
    types: Float;

    impl ::Conversion<V> for V {
        type T = V;

        #[inline(always)]
        fn into_conversion(&self) -> Self::T {
            *self
        }
    }

    impl ::ConversionFactor<V> for V {
        #[inline(always)]
        fn powi(self, e: i32) -> Self {
            self.powi(e)
        }

        #[inline(always)]
        fn value(self) -> V {
            self
        }
    }
}

storage_types! {
    types: PrimInt;

    impl ::Conversion<V> for V {
        type T = ::num::rational::Ratio<V>;

        #[inline(always)]
        fn into_conversion(&self) -> Self::T {
            (*self).into()
        }
    }

    impl ::ConversionFactor<V> for ::num::rational::Ratio<V> {
        #[inline(always)]
        fn powi(self, e: i32) -> Self {
            self.pow(e)
        }

        #[inline(always)]
        fn value(self) -> V {
            self.to_integer()
        }
    }
}

storage_types! {
    types: BigInt, BigUint;

    impl ::Conversion<V> for V {
        type T = ::num::rational::Ratio<V>;

        #[inline(always)]
        fn into_conversion(&self) -> Self::T {
            self.clone().into()
        }
    }

    impl ::ConversionFactor<V> for ::num::rational::Ratio<V> {
        #[inline(always)]
        fn powi(self, e: i32) -> Self {
            match e.cmp(&0) {
                ::lib::cmp::Ordering::Equal => <Self as ::num::One>::one(),
                ::lib::cmp::Ordering::Less => ::num::pow::pow(self.recip(), (-e) as usize),
                ::lib::cmp::Ordering::Greater => ::num::pow::pow(self, e as usize),
            }
        }

        #[inline(always)]
        fn value(self) -> V {
            self.to_integer()
        }
    }
}

storage_types! {
    types: Rational, Rational32, Rational64;

    impl ::Conversion<V> for V {
        type T = V;

        #[inline(always)]
        fn into_conversion(&self) -> Self::T {
            *self
        }
    }

    impl ::ConversionFactor<V> for V {
        #[inline(always)]
        fn powi(self, e: i32) -> Self {
            self.pow(e)
        }

        #[inline(always)]
        fn value(self) -> V {
            self
        }
    }
}

storage_types! {
    types: BigRational;

    impl ::Conversion<V> for V {
        type T = V;

        #[inline(always)]
        fn into_conversion(&self) -> Self::T {
            self.clone()
        }
    }

    impl ::ConversionFactor<V> for V {
        #[inline(always)]
        fn powi(self, e: i32) -> Self {
            match e.cmp(&0) {
                ::lib::cmp::Ordering::Equal => <Self as ::num::One>::one(),
                ::lib::cmp::Ordering::Less => ::num::pow::pow(self.recip(), (-e) as usize),
                ::lib::cmp::Ordering::Greater => ::num::pow::pow(self, e as usize),
            }
        }

        #[inline(always)]
        fn value(self) -> V {
            self
        }
    }
}
