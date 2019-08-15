use super::*;
use proc_macro2::TokenStream;
use quote::quote;

/// Expand the `quantity!` macro invocation to define the parsed quantity and associated units.
pub(crate) fn expand(input: Quantity) -> Result<TokenStream, syn::Error> {
    let Quantity {
        name_attributes,
        name,
        // description,
        // dimension_attributes,
        // system,
        // dimensions,
        // kind,
        // units,
        ..
    } = input;

    let ts = quote! {
        #(#name_attributes)*
        pub type #name = uom::Quantity;
    };

    Ok(ts)
}
