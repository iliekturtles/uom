//! Units of measurement is a crate that does automatic type-safe zero-cost
//! [dimensional analysis][analysis]. The [International System of Units][si] (SI) is implemented
//! based on the [International System of Quantities][isq] (ISQ) and includes numerous
//! [quantities][quantity] (length, mass, time, ...) with conversion factors for even more numerous
//! [measurement units][measurement] (meter, kilometer, foot, mile, ...). No more crashing your
//! [climate orbiter][orbiter]!
//!
//! [analysis]: https://en.wikipedia.org/wiki/Dimensional_analysis
//! [si]: http://jcgm.bipm.org/vim/en/1.16.html
//! [isq]: http://jcgm.bipm.org/vim/en/1.6.html
//! [quantity]: http://jcgm.bipm.org/vim/en/1.1.html
//! [measurement]: http://jcgm.bipm.org/vim/en/1.9.html
//! [orbiter]: https://en.wikipedia.org/wiki/Mars_Climate_Orbiter

#![no_std]
#![warn(missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unused_import_braces,
    unused_qualifications)]

// Clippy.
#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

#[doc(hidden)]
pub extern crate typenum;

#[doc(hidden)]
pub mod stdlib {
    pub use core::*;
}

#[macro_use]
mod system;

#[cfg(feature = "si")]
pub mod si;
