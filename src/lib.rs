//! Units of measurement is a crate that does automatic type-safe zero-cost
//! [dimensional analysis][analysis]. You can create your own systems or use the pre-built
//! [International System of Units][si] (SI) which is based on the
//! [International System of Quantities][isq] (ISQ) and includes numerous [quantities][quantity]
//! (length, mass, time, ...) with conversion factors for even more numerous
//! [measurement units][measurement] (meter, kilometer, foot, mile, ...). No more crashing your
//! [climate orbiter][orbiter]!
//!
//! [analysis]: https://en.wikipedia.org/wiki/Dimensional_analysis
//! [si]: https://jcgm.bipm.org/vim/en/1.16.html
//! [isq]: https://jcgm.bipm.org/vim/en/1.6.html
//! [quantity]: https://jcgm.bipm.org/vim/en/1.1.html
//! [measurement]: https://jcgm.bipm.org/vim/en/1.9.html
//! [orbiter]: https://en.wikipedia.org/wiki/Mars_Climate_Orbiter
//!
//! ## Usage
//! `uom` requires `rustc` 1.43.0 or later. Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! uom = "0.33.0"
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
//!     let velocity/*: Velocity*/ = length / time;
//!     let _acceleration = calc_acceleration(velocity, time);
//!     //let error = length + time; // error[E0308]: mismatched types
//! }
//!
//! fn calc_acceleration(velocity: Velocity, time: Time) -> Acceleration {
//!     velocity / time
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
//!     version = "0.33.0",
//!     default-features = false,
//!     features = [
//!         "autoconvert", # automatic base unit conversion.
//!         "usize", "u8", "u16", "u32", "u64", "u128", # Unsigned integer storage types.
//!         "isize", "i8", "i16", "i32", "i64", "i128", # Signed integer storage types.
//!         "bigint", "biguint", # Arbitrary width integer storage types.
//!         "rational", "rational32", "rational64", "bigrational", # Integer ratio storage types.
//!         "complex32", "complex64", # Complex floating point storage types.
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
//!  * `usize`, `u8`, `u16`, `u32`, `u64`, `u128`, `isize`, `i8`, `i16`, `i32`, `i64`, `i128`,
//!    `bigint`, `biguint`, `rational`, `rational32`, `rational64`, `bigrational`, `complex32`,
//!    `complex64`, `f32`, `f64` -- Features to enable underlying storage types. At least one of
//!    these features must be enabled. `f32` and `f64` are enabled by default. See the
//!    [Design](#design) section for implications of choosing different underlying storage types.
//!  * `si` -- Feature to include the pre-built [International System of Units][si] (SI). Enabled by
//!    default.
//!  * `std` -- Feature to compile with standard library support. Disabling this feature compiles
//!    `uom` with `no_std`. Enabled by default.
//!  * `use_serde` -- Feature to enable support for serialization and deserialization of quantities
//!    with the [Serde][serde] crate. Disabled by default.
//!
//!    [Serde][serde] support for the `big*` and `rational*` underlying storage types requires
//!    manually enabling the `serde` feature for the `num-rational` and `num-bigint` crates. To do
//!    so, you can add one or both of the following lines to your `Cargo.toml`:
//!
//!    ```toml
//!    num-rational = { version = "*", features = ["serde"] }
//!    num-bigint = { version = "*", features = ["serde"] }
//!    ```
//!
//! [si]: https://jcgm.bipm.org/vim/en/1.16.html
//! [serde]: https://serde.rs/
//!
//! ## Design
//! Rather than working with [measurement units](https://jcgm.bipm.org/vim/en/1.9.html) (meter,
//! kilometer, foot, mile, ...) `uom` works with [quantities](https://jcgm.bipm.org/vim/en/1.1.html)
//! (length, mass, time, ...). This simplifies usage because units are only involved at interface
//! boundaries: the rest of your code only needs to be concerned about the quantities involved.
//! This also makes operations on quantities (+, -, \*, /, ...) have zero runtime cost over using
//! the raw storage type (e.g. `f32`).
//!
//! `uom` normalizes values to the [base unit](https://jcgm.bipm.org/vim/en/1.10.html) for the
//! quantity. Alternative base units can be used by executing the macro defined for the system of
//! quantities (`ISQ!` for the SI). `uom` supports `usize`, `u8`, `u16`, `u32`, `u64`, `u128`,
//! `isize`, `i8`, `i16`, `i32`, `i64`, `i128`, `bigint`, `biguint`, `rational`, `rational32`,
//! `rational64`, `bigrational`, `complex32`, `complex64`, `f32`, and `f64` as the underlying
//! storage type.
//!
//! A consequence of normalizing values to the base unit is that some values may not be able to be
//! represented or can't be precisely represented for floating point and rational underlying
//! storage types. For example if the base unit of `length` is `meter` and the underlying storage
//! type is `i32` then values like `1 centimeter` or `1.1 meter` cannot be represented. `1
//! centimeter` is normalized to `0.01 meter` which can't be stored in an `i32`. `uom` only allows
//! units to be used safely. Users of this library will still need to be aware of implementation
//! details of the underlying storage type including limits and precision.
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
//!    <https://www.apache.org/licenses/LICENSE-2.0>)
//!  * MIT license ([LICENSE-MIT](LICENSE-MIT) or <https://opensource.org/licenses/MIT>)
//!
//! at your option.
//!
//! [BIPM]: https://www.bipm.org/en/about-us/
//! [brochure]: https://www.bipm.org/en/publications/si-brochure/
//! [si]: https://jcgm.bipm.org/vim/en/1.16.html
//! [nist811]: https://www.nist.gov/pml/nist-guide-si-appendix-b9-factors-units-listed-kind-quantity-or-field-science

// Compile with `no_std` when the `std` feature is not specified.
#![cfg_attr(not(feature = "std"), no_std)]
// Rustc lints.
#![forbid(unsafe_code)]
#![warn(
    bare_trait_objects,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]
// Clippy lints.
#![cfg_attr(
    feature = "cargo-clippy",
    warn(
        clippy::must_use_candidate,
        clippy::return_self_not_must_use,
    ),
    allow(
        clippy::deprecated_cfg_attr,
        clippy::excessive_precision,
        clippy::inconsistent_digit_grouping, // https://github.com/rust-lang/rust-clippy/issues/6096
        clippy::inline_always,
    )
)]
// Lints allowed in tests because they are unavoidable in the generic code when a type may or may
// not need to be dereferenced or cloned.
#![cfg_attr(
    all(feature = "cargo-clippy", test),
    allow(clippy::op_ref, clippy::clone_on_copy, clippy::float_cmp)
)]

// Fail to compile if no underlying storage type features are specified.
#[rustfmt::skip]
#[cfg(not(any(
    feature = "usize", feature = "u8", feature = "u16", feature = "u32", feature = "u64",
    feature = "u128",
    feature = "isize", feature = "i8", feature = "i16", feature = "i32", feature = "i64",
    feature = "i128",
    feature = "bigint", feature = "biguint",
    feature = "rational", feature = "rational32", feature = "rational64", feature = "bigrational",
    feature = "complex32", feature = "complex64",
    feature = "f32", feature = "f64", )))]
compile_error!("A least one underlying storage type must be enabled. See the features section of \
    uom documentation for available underlying storage type options.");

#[doc(hidden)]
pub extern crate num_traits;

#[doc(hidden)]
#[cfg(feature = "bigint-support")]
pub extern crate num_bigint;

#[doc(hidden)]
#[cfg(any(feature = "rational-support", feature = "bigint-support"))]
pub extern crate num_rational;

#[doc(hidden)]
#[cfg(feature = "complex-support")]
pub extern crate num_complex;

#[doc(hidden)]
#[cfg(feature = "serde")]
pub extern crate serde;

#[doc(hidden)]
pub extern crate typenum;

#[cfg(all(
    test,
    any(feature = "f32", feature = "f64", feature = "complex32", feature = "complex64")
))]
#[macro_use]
extern crate approx;
#[cfg(test)]
#[macro_use]
extern crate quickcheck;
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

    // Export `panic` module when the `std` feature is not enabled. `RefUnwindSafe` and `UnwindSafe`
    // traits do not exist in `core` but are conditionally needed in traits defined by `uom` when
    // `std` is enabled. These definitions work around conditional requirements.
    #[cfg(not(feature = "std"))]
    pub mod panic {
        pub trait RefUnwindSafe {}
        pub trait UnwindSafe {}
    }
}

// Conditionally import num sub-crate types based on feature selection.
#[doc(hidden)]
pub mod num {
    #[cfg(feature = "std")]
    pub use num_traits::float::Float;
    #[cfg(not(feature = "std"))]
    pub use num_traits::float::FloatCore as Float;

    pub use num_traits::{pow, FromPrimitive, Num, One, Saturating, Signed, ToPrimitive, Zero};

    #[cfg(feature = "bigint-support")]
    pub use num_bigint::{BigInt, BigUint};

    #[cfg(feature = "rational-support")]
    pub type Rational = num_rational::Ratio<isize>;

    #[cfg(feature = "bigint-support")]
    pub use num_rational::BigRational;

    #[cfg(any(feature = "rational-support", feature = "bigint-support"))]
    pub mod rational {
        pub use num_rational::*;
    }

    #[cfg(feature = "complex-support")]
    pub mod complex {
        pub use num_complex::*;
    }
}

/// Primitive traits and types representing basic properties of types.
pub mod marker {
    /// Trait to denote that a quantity is able to be added with a quantity of the same dimensions.
    /// When a specific quantity's kind inherits this trait `ops::Add` is implemented
    /// automatically.
    pub trait Add {}

    /// Trait to denote that a quantity is able to be added with a quantity of the same dimensions.
    /// When a specific quantity's kind inherits this trait `ops::AddAssign` is implemented
    /// automatically.
    pub trait AddAssign {}

    /// Trait to denote that a quantity is able to be subtracted with a quantity of the same
    /// dimensions. When a specific quantity's kind inherits this trait `ops::Sub` is implemented
    /// automatically.
    pub trait Sub {}

    /// Trait to denote that a quantity is able to be subtracted with a quantity of the same
    /// dimensions. When a specific quantity's kind inherits this trait `ops::SubAssign` is
    /// implemented automatically.
    pub trait SubAssign {}

    /// Trait to denote that a quantity is able to be multiplied with a quantity of the same
    /// dimensions. When a specific quantity's kind inherits this trait `ops::Mul` is implemented
    /// automatically.
    pub trait Mul {}

    /// Trait to denote that a quantity is able to be multiplied with a quantity of the same
    /// dimensions. When a specific quantity's kind inherits this trait `ops::MulAssign` is
    /// implemented automatically.
    pub trait MulAssign {}

    /// Trait to denote that a quantity is able to be divided with a quantity of the same
    /// dimensions. When a specific quantity's kind inherits this trait `ops::Div` is implemented
    /// automatically.
    pub trait Div {}

    /// Trait to denote that a quantity is able to be divided with a quantity of the same
    /// dimensions. When a specific quantity's kind inherits this trait `ops::DivAssign` is
    /// implemented automatically.
    pub trait DivAssign {}

    /// Trait to denote that a quantity is able to be negated. When a specific quantity's kind
    /// inherits this trait `ops::Neg` is implemented automatically.
    pub trait Neg {}

    /// Trait to denote that a quantity is able to calculate a remainder with a quantity of the
    /// same dimensions. When a specific quantity's kind inherits this trait `ops::Rem` is
    /// implemented automatically.
    pub trait Rem {}

    /// Trait to denote that a quantity is able to calculate a remainder with a quantity of the
    /// same dimensions. When a specific quantity's kind inherits this trait `ops::RemAssign` is
    /// implemented automatically.
    pub trait RemAssign {}

    /// Trait to denote that a quantity is able to perform saturating additions and subtractions
    /// with a quantity of the same dimensions. When a specific quantity's kind inherits this trait
    /// `ops::Saturating` is implemented automatically.
    pub trait Saturating {}
}

#[macro_use]
mod features;

#[macro_use]
mod storage_types;

#[macro_use]
mod system;

#[macro_use]
mod quantity;

#[macro_use]
mod unit;

#[cfg(feature = "si")]
#[macro_use]
pub mod si;

#[cfg(test)]
mod tests;

/// Operations performed on the constant portion of the [conversion factor][factor]. Used to help
/// guide optimizations when floating point underlying storage types are used.
///
/// For a value, `v: Float`, adding `-0.0` is a no-op while adding `0.0` will change the sign if
/// `v` is `-0.0`. The opposite is true for subtraction.
///
/// ```ignore
///    v
///  0.0 + -0.0 =  0.0
/// -0.0 +  0.0 =  0.0 // v +  0.0 != v
/// -0.0 + -0.0 = -0.0
///  0.0 - -0.0 =  0.0
/// -0.0 -  0.0 =  0.0
/// -0.0 - -0.0 =  0.0 // v - -0.0 != v
/// ```
///
/// [factor]: https://jcgm.bipm.org/vim/en/1.24.html
#[derive(Clone, Copy, Debug)]
pub enum ConstantOp {
    /// Hint that the constant is being added to a value.
    Add,

    /// Hint that the constant is being subtracted from a value.
    Sub,
}

/// Trait to identify [units][units] which have a [conversion factor][factor].
///
/// ## Generic Parameters
/// * `V`: Underlying storage type trait is implemented for.
///
/// [units]: https://jcgm.bipm.org/vim/en/1.13.html
/// [factor]: https://jcgm.bipm.org/vim/en/1.24.html
pub trait Conversion<V> {
    /// Conversion factor type specific to the underlying storage type.
    type T: ConversionFactor<V>;

    /// Coefficient portion of [conversion factor](https://jcgm.bipm.org/vim/en/1.24.html) for
    /// converting the given unit to the base unit for the quantity: `(value * coefficient()) +
    /// constant()`. Implementation should return the multiplicative identity (`Self::T::one()`) if
    /// no coefficient exists.
    #[must_use = "method returns a new number and does not mutate the original value"]
    #[inline(always)]
    fn coefficient() -> Self::T {
        <Self::T as crate::num::One>::one()
    }

    /// Constant portion of [conversion factor](https://jcgm.bipm.org/vim/en/1.24.html) for
    /// converting the given unit to the base unit for the quantity: `(value * coefficient()) +
    /// constant()`. Implementation should return the additive identity (`Self::T::zero()`) if no
    /// constant exists. See [ConstantOp](enum.ConstantOp.html) documentation for details about
    /// parameter use to ensure the method optimizes correctly.
    #[must_use = "method returns a new number and does not mutate the original value"]
    #[inline(always)]
    #[allow(unused_variables)]
    fn constant(op: ConstantOp) -> Self::T {
        <Self::T as crate::num::Zero>::zero()
    }

    /// Instance [conversion factor](https://jcgm.bipm.org/vim/en/1.24.html).
    ///
    /// Default implementation returns the coefficient: `Self::coefficient()`.
    #[must_use = "method returns a new number and does not mutate the original value"]
    #[inline(always)]
    fn conversion(&self) -> Self::T
    where
        Self: Sized,
    {
        Self::coefficient()
    }
}

/// Trait representing a [conversion factor][factor].
///
/// ## Generic Parameters
/// * `V`: Underlying storage type trait is implemented for.
///
/// [factor]: https://jcgm.bipm.org/vim/en/1.24.html
#[allow(unused_qualifications)] // lib:cmp::PartialOrder false positive.
pub trait ConversionFactor<V>:
    lib::cmp::PartialOrd
    + lib::ops::Add<Self, Output = Self>
    + lib::ops::Sub<Self, Output = Self>
    + lib::ops::Mul<Self, Output = Self>
    + lib::ops::Div<Self, Output = Self>
    + crate::num::Zero
    + crate::num::One
{
    /// Raises a `ConversionFactor<V>` to an integer power.
    #[must_use = "method returns a new number and does not mutate the original value"]
    fn powi(self, e: i32) -> Self;

    /// Converts a `ConversionFactor<V>` into its underlying storage type.
    #[must_use = "method returns a new number and does not mutate the original value"]
    fn value(self) -> V;
}

/// Helper trait to identify the zero value of a type at compile time.
///
#[cfg_attr(all(feature = "si", feature = "f32"), doc = " ```rust")]
#[cfg_attr(not(all(feature = "si", feature = "f32")), doc = " ```rust,ignore")]
/// # use uom::si::f32::Length;
/// use uom::ConstZero;
///
/// const ORIGIN: (Length, Length, Length) = (Length::ZERO, Length::ZERO, Length::ZERO);
/// ```
pub trait ConstZero {
    /// Constant representing the zero value.
    const ZERO: Self;
}

/// Default [kind][kind] of quantities to allow addition, subtraction, multiplication, division,
/// remainder, negation, and saturating addition/subtraction.
///
/// [kind]: https://jcgm.bipm.org/vim/en/1.2.html
pub trait Kind:
    marker::Add
    + marker::AddAssign
    + marker::Sub
    + marker::SubAssign
    + marker::Mul
    + marker::MulAssign
    + marker::Div
    + marker::DivAssign
    + marker::Rem
    + marker::RemAssign
    + marker::Neg
    + marker::Saturating
{
}

storage_types! {
    types: Float;

    impl crate::Conversion<V> for V {
        type T = V;

        #[inline(always)]
        fn constant(op: crate::ConstantOp) -> Self::T {
            match op {
                crate::ConstantOp::Add => -<Self::T as crate::num::Zero>::zero(),
                crate::ConstantOp::Sub => <Self::T as crate::num::Zero>::zero(),
            }
        }

        #[inline(always)]
        fn conversion(&self) -> Self::T {
            *self
        }
    }

    impl crate::ConversionFactor<V> for V {
        #[inline(always)]
        fn powi(self, e: i32) -> Self {
            <V as crate::num::Float>::powi(self, e)
        }

        #[inline(always)]
        fn value(self) -> V {
            self
        }
    }

    impl crate::ConstZero for V {
        const ZERO: Self = 0.0;
    }
}

storage_types! {
    types: PrimInt;

    impl crate::Conversion<V> for V {
        type T = crate::num::rational::Ratio<V>;

        #[inline(always)]
        fn conversion(&self) -> Self::T {
            (*self).into()
        }
    }

    impl crate::ConversionFactor<V> for crate::num::rational::Ratio<V> {
        #[inline(always)]
        fn powi(self, e: i32) -> Self {
            self.pow(e)
        }

        #[inline(always)]
        fn value(self) -> V {
            self.to_integer()
        }
    }

    impl crate::ConstZero for V {
        const ZERO: Self = 0;
    }
}

storage_types! {
    types: BigInt, BigUint;

    impl crate::Conversion<V> for V {
        type T = crate::num::rational::Ratio<V>;

        #[inline(always)]
        fn conversion(&self) -> Self::T {
            self.clone().into()
        }
    }

    impl crate::ConversionFactor<V> for crate::num::rational::Ratio<V> {
        #[inline(always)]
        fn powi(self, e: i32) -> Self {
            match e.cmp(&0) {
                crate::lib::cmp::Ordering::Equal => <Self as crate::num::One>::one(),
                crate::lib::cmp::Ordering::Less => crate::num::pow::pow(self.recip(), (-e) as usize),
                crate::lib::cmp::Ordering::Greater => crate::num::pow::pow(self, e as usize),
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

    impl crate::Conversion<V> for V {
        type T = V;

        #[inline(always)]
        fn conversion(&self) -> Self::T {
            *self
        }
    }

    impl crate::ConversionFactor<V> for V {
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

    impl crate::Conversion<V> for V {
        type T = V;

        #[inline(always)]
        fn conversion(&self) -> Self::T {
            self.clone()
        }
    }

    impl crate::ConversionFactor<V> for V {
        #[inline(always)]
        fn powi(self, e: i32) -> Self {
            match e.cmp(&0) {
                crate::lib::cmp::Ordering::Equal => <Self as crate::num::One>::one(),
                crate::lib::cmp::Ordering::Less => crate::num::pow::pow(self.recip(), (-e) as usize),
                crate::lib::cmp::Ordering::Greater => crate::num::pow::pow(self, e as usize),
            }
        }

        #[inline(always)]
        fn value(self) -> V {
            self
        }
    }
}

storage_types! {
    types: Complex;
    impl crate::Conversion<V> for V {
        type T = VV;

        #[inline(always)]
        fn constant(op: crate::ConstantOp) -> Self::T {
            match op {
                crate::ConstantOp::Add => -<Self::T as crate::num::Zero>::zero(),
                crate::ConstantOp::Sub => <Self::T as crate::num::Zero>::zero(),
            }
        }

        #[inline(always)]
        fn conversion(&self) -> Self::T {
            // Conversion factor is the norm of the number. Scaling with length again yields the
            // same number.
            self.norm()
        }
    }

    impl crate::ConversionFactor<V> for VV {
        #[inline(always)]
        fn powi(self, e: i32) -> Self {
            self.powi(e)
        }

        #[inline(always)]
        fn value(self) -> V {
            // Conversion by scaling (multiplication with only real number). Scaling a normalized
            // number yields the original number again.
            V::new(self, 0.0)
        }
    }
}

/// Utilities for formatting and printing quantities.
pub mod fmt {
    /// An enum to specify the display style to use.
    #[derive(Clone, Copy, Debug)]
    pub enum DisplayStyle {
        /// Display the value and a unit abbreviation, e.g. "1.0 m", "327 s".
        Abbreviation,

        /// Display the value and full unit name (pluralized as appropriate),
        /// e.g. "1 kilogram", "756 feet".
        Description,
    }
}

/// Unicode string slice manipulation for quantities.
pub mod str {
    use crate::lib::fmt::{self, Display, Formatter};

    /// Represents an error encountered while parsing a string into a `Quantity`.
    #[allow(missing_copy_implementations)]
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum ParseQuantityError {
        /// No separators (spaces) were encountered.
        NoSeparator,

        /// An error occurred while parsing the value (first) portion of the string.
        ///
        /// Due to exhaustiveness and type system limitations, this variant does not encode
        /// the underlying parse error.
        ValueParseError,

        /// The unit used wasn't found for this quantity.
        ///
        /// ### Notes
        /// For now, only abbreviations are supported, so this error may be encountered even if the
        /// unit name (description) is correct.
        UnknownUnit,
    }

    impl Display for ParseQuantityError {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            use ParseQuantityError::*;

            match *self {
                NoSeparator => write!(f, "no space between quantity and units"),
                ValueParseError => write!(f, "error parsing unit quantity"),
                UnknownUnit => write!(f, "unrecognized unit of measure"),
            }
        }
    }

    #[cfg(feature = "std")]
    impl crate::lib::error::Error for ParseQuantityError {}
}
