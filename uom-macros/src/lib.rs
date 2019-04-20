//! ⚠ This crate is currently a placeholder for future development efforts. ⚠
//!
//! `uom-macros` provides procedural macro support for `uom`. Two function-style macros are
//! available. `system!`, to define a system of quantities and a related system of units, and
//! `quantity!`, to define quantities within a system.  See the
//! [`uom`](https://crates.io/crates/uom) crate for full details.

// Rustc lints.
#![forbid(unsafe_code)]
#![warn(
    //bare_trait_objects, // Requires rustc 1.27.
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

extern crate proc_macro;

use crate::proc_macro::TokenStream;

/// Define a system of quantities.
#[proc_macro]
pub fn system(input: TokenStream) -> TokenStream {
    input
}

/// Define a quantity within a system of quantities.
#[proc_macro]
pub fn quantity(input: TokenStream) -> TokenStream {
    input
}
