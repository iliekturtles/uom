use syn::punctuated::Punctuated;
use syn::{Attribute, Ident, Path, Token};

mod expand;
mod parse;

/// Expand the `system!` macro invocation.
pub(crate) use expand::expand;

/// System definition. TODO: Remove `allow(dead_code)` once parsing/expansion implemented.
#[allow(dead_code)]
#[cfg_attr(feature = "extra-traits", derive(Debug))]
pub(crate) struct System {
    name_attributes: Vec<Attribute>,
    name: Ident,
    units_attributes: Vec<Attribute>,
    units: Ident,
    base_quantities: Punctuated<BaseQuantity, Token![;]>,
    quantities: Punctuated<Quantity, Token![;]>,
}

/// Base quantity definition. TODO: Remove `allow(dead_code)` once parsing/expansion implemented.
#[allow(dead_code)]
#[cfg_attr(feature = "extra-traits", derive(Debug))]
pub(crate) struct BaseQuantity {
    attributes: Vec<Attribute>,
    name: Ident,
    symbol: Ident,
}

/// Quantity reference defintion. TODO: Remove `allow(dead_code)` once parsing/expansion
/// implemented.
#[allow(dead_code)]
#[cfg_attr(feature = "extra-traits", derive(Debug))]
pub(crate) struct Quantity {
    add_mod: bool,
    module: Path,
    quantity: Ident,
}
