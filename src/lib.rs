//! Units of measurement is a crate that does automatic type-safe zero-cost dimensional analysis.
//! The International System of Units (SI) is implemented based on the International System of
//! Quantities (ISQ) and includes numerous quantities (length, mass, time, ...) with conversion
//! factors for even more numerous measurement units (meter, kilometer, foot, mile, ...). No more
//! crashing your [climate orbiter](https://en.wikipedia.org/wiki/Mars_Climate_Orbiter)!

#![cfg_attr(not(feature = "std"), no_std)]
//#![warn(missing_docs)]

// Clippy.
#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

#[doc(hidden)]
#[cfg(feature = "std")]
pub mod stdlib {
    pub use std::*;
}

#[doc(hidden)]
#[cfg(not(feature = "std"))]
pub mod stdlib {
    pub use core::*;
}
/// Property of a phenomenon, body or substance, where the property has a magnitude that can be
/// expressed as a number and a reference.
///
/// * http://jcgm.bipm.org/vim/en/1.1.html
#[derive(Copy, Clone, Debug)]
pub struct Quantity<D, U, V>
    where D: Dimension,
          U: Units<D, V>,
{
    value: V,
    dimension: stdlib::marker::PhantomData<D>,
    units: stdlib::marker::PhantomData<U>,
}

/// Marker trait to express the dependence of a quantity on the base quantities of a system of
/// quantities as a product of powers of factors corresponding to the base quantities, omitting any
/// numerical factor.
///
/// * http://jcgm.bipm.org/vim/en/1.7.html
pub trait Dimension {}

/// Trait to identify a system of units based on a set of base units of a system of quantities.
///
/// * http://jcgm.bipm.org/vim/en/1.10.html
/// * http://jcgm.bipm.org/vim/en/1.13.html
pub trait Units<D, V>
    where D: Dimension,
{
    /// Conversion factor for the given base units to the base units for the system of quantities.
    fn conversion() -> V;
}

/// Trait to identify measurement units of individual quantities.
///
/// * http://jcgm.bipm.org/vim/en/1.9.html
pub trait Unit<V> {
    /// Conversion factor for the given unit to the base unit for the quantity.
    fn conversion() -> V;
}
