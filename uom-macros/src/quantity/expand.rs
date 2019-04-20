use super::*;
use proc_macro2::TokenStream;

/// Expand the `quantity!` macro invocation to define the parsed quantity and associated units.
pub(crate) fn expand(input: Quantity) -> Result<TokenStream, syn::Error> {
    drop(input);

    Ok(TokenStream::new())
}
