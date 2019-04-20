use super::*;
use proc_macro2::TokenStream;

/// Expand the `system!` macro invocation to define the system of quantities and default system of
/// units.
pub(crate) fn expand(input: System) -> Result<TokenStream, syn::Error> {
    drop(input);

    Ok(TokenStream::new())
}
