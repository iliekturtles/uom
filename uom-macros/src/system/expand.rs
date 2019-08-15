use super::*;
use proc_macro2::TokenStream;
use quote::quote;

/// Expand the `system!` macro invocation to define the system of quantities and default system of
/// units.
pub(crate) fn expand(input: System) -> Result<TokenStream, syn::Error> {
    let System {
        // name_attributes,
        // name,
        // units_attributes,
        // units,
        base_quantities,
        quantities,
        ..
    } = input;
    let dimensions = base_quantities.iter().map(|bq| {
        let attributes = &bq.attributes;
        let symbol = &bq.symbol;

        quote! {
            #(#attributes)*
            type #symbol: typenum::Integer;
        }
    });
    let mods = quantities.iter().filter_map(|q| {
        if q.add_mod {
            let module = &q.module;

            Some(quote! { pub mod #module; })
        } else {
            None
        }
    });

    let ts = quote! {
        /// TODO
        pub trait Dimension: Send + Sync {
            #(#dimensions)*
        }

        #(#mods)*
    };

    Ok(ts)
}
