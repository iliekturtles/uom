//! Validate that the `feature_check` crate has no `std` feature and `system!` still generates code
//! for the `std` feature.
//!
//! ```
//! #[macro_use]
//! extern crate uom;
//!
//! #[cfg(feature = "std")]
//! compile_error!("Unexpected feature `std` in feature_check test crate.");
//!
//! #[macro_use]
//! mod area {
//!     quantity! {
//!         quantity: Area; "area";
//!         dimension: Q<P2>;
//!         units {
//!             @square_meter: 1.0E0; "m²", "square meter", "square meters";
//!         }
//!     }
//! }
//!
//! #[macro_use]
//! mod length {
//!     quantity! {
//!         quantity: Length; "length";
//!         dimension: Q<P1>;
//!         units {
//!             @meter: 1.0E0; "m", "meter", "meters";
//!         }
//!     }
//! }
//!
//! system! {
//!     quantities: Q {
//!         length: meter, L;
//!     }
//!
//!     units: U {
//!         mod area::Area,
//!         mod length::Length,
//!     }
//! }
//!
//! mod f32 {
//!     mod m {
//!         pub use super::super::*;
//!     }
//!
//!     Q!(self::m, f32);
//! }
//!
//! fn main() {
//!     let l1 = f32::Area::new::<area::square_meter>(4.0).sqrt();
//! }
//! ```
//!
//! Validate that the `feature_check` crate has no `f32` feature and `storage_types!` still
//! generates code for the `f32` type.
//!
//! ```
//! #[macro_use]
//! extern crate uom;
//!
//! #[cfg(feature = "f32")]
//! compile_error!("Unexpected feature `f32` in feature_check test crate.");
//!
//! storage_types! {
//!     types: i32, f32;
//!
//!     pub fn do_work(v: V) {}
//! }
//!
//! fn main() {
//!     crate::f32::do_work(1.0);
//! }
//! ```
//!
//! Validate that `storage_types!` does not generate code for the `i32` type.
//!
//! ```rust,compile_fail
//! #[macro_use]
//! extern crate uom;
//!
//! storage_types! {
//!     types: i32, f32;
//!
//!     pub fn do_work(v: V) {}
//! }
//!
//! fn main() {
//!     crate::i32::do_work(1.0);
//! }
//! ```
