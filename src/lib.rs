//! Units of measurement is a crate that does automatic type-safe zero-cost
//! [dimensional analysis][analysis]. You can create your own systems or use the pre-built
//! [International System of Units][si] (SI) which is based on the
//! [International System of Quantities][isq] (ISQ) and includes numerous [quantities][quantity]
//! (length, mass, time, ...) with conversion factors for even more numerous
//! [measurement units][measurement] (meter, kilometer, foot, mile, ...). No more crashing your
//! [climate orbiter][orbiter]!
//!
//! The simple example below shows how to use quantities and units as well as how `uom` stops
//! invalid operations
//!
//! ```
//! extern crate uom;
//!
//! use uom::si::f32::*;
//! use uom::si::length::kilometer;
//! use uom::si::time::second;
//!
//! fn main() {
//! 	let length = Length::new::<kilometer>(5.0);
//! 	let time = Time::new::<second>(15.0);
//! 	let _velocity = length / time;
//! 	//let error = length + time; // error[E0308]: mismatched types
//! }
//! ```
//!
//! See examples provided with the crate for more advanced usage including how to create `Quantity`
//! type aliases for a different set of base units and how to create an entirely new system.
//!
//! [analysis]: https://en.wikipedia.org/wiki/Dimensional_analysis
//! [si]: http://jcgm.bipm.org/vim/en/1.16.html
//! [isq]: http://jcgm.bipm.org/vim/en/1.6.html
//! [quantity]: http://jcgm.bipm.org/vim/en/1.1.html
//! [measurement]: http://jcgm.bipm.org/vim/en/1.9.html
//! [orbiter]: https://en.wikipedia.org/wiki/Mars_Climate_Orbiter

#![no_std]
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

// Clippy.
#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

#[doc(hidden)]
pub extern crate typenum;
#[cfg(test)]
#[macro_use]
extern crate quickcheck;

#[doc(hidden)]
pub mod stdlib {
    pub use core::*;
}

#[macro_use]
mod system;

#[cfg(feature = "si")]
#[macro_use]
pub mod si;

#[cfg(test)]
mod tests;
