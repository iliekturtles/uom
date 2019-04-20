use super::*;
use syn::parse::{Parse, ParseStream};
use proc_macro2::Span;

impl Parse for System {
    fn parse(_input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            name_attributes: vec![],
            name: Ident::new("name", Span::call_site()),
            units_attributes: vec![],
            units: Ident::new("units", Span::call_site()),
            base_quantities: Punctuated::new(),
            quantities: Punctuated::new(),
        })
    }
}

impl Parse for BaseQuantity {
    fn parse(_input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            attributes: vec![],
            name: Ident::new("name", Span::call_site()),
            symbol: Ident::new("symbol", Span::call_site()),
        })
    }
}

impl Parse for Quantity {
    fn parse(_input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            add_mod: false,
            module: syn::parse_str("quantity::Quantity")?,
            quantity: Ident::new("quantity", Span::call_site()),
        })
    }
}
