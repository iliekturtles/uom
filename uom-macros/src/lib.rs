//! ⚠ This crate is currently a placeholder for future development efforts. ⚠
//!
//! `uom-macros` provides procedural macro support for `uom`. Two function-style macros are
//! available. `system!`, to define a system of quantities and a related system of units, and
//! `quantity!`, to define quantities within a system.  See the
//! [`uom`](https://crates.io/crates/uom) crate for full details.

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
