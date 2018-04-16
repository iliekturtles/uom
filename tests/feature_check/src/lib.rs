//! Validate that the feature_check crate has no `f32` feature and `storage_types!` still generates
//! code for the `f32` type.
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
//!     ::f32::do_work(1.0);
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
//!     ::i32::do_work(1.0);
//! }
//! ```
