use super::*;
use syn::parse::{Parse, ParseStream};
use proc_macro2::Span;

impl Parse for Quantity {
    fn parse(_input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            name_attributes: vec![],
            name: Ident::new("name", Span::call_site()),
            description: syn::parse_str("\"description\"")?,
            dimension_attributes: vec![],
            system: Ident::new("system", Span::call_site()),
            dimensions: Punctuated::new(),
            kind: None,
            units: Punctuated::new(),
        })
    }
}

impl Parse for Unit {
    fn parse(_input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            attributes: vec![],
            name: Ident::new("name", Span::call_site()),
            coefficient: syn::parse_str("1.0")?,
            constant: None,
            abbreviation: syn::parse_str("\"abbreviation\"")?,
            singular: syn::parse_str("\"singular\"")?,
            plural: syn::parse_str("\"plural\"")?,
        })
    }
}
