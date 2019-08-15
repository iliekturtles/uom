use super::*;
use crate::parse::ParseStreamExt;
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::{Attribute, Error, Token};

impl Parse for System {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            name_attributes: input.call(Attribute::parse_outer)?,
            name: input.parse_field("name")?,
            units_attributes: input.call(Attribute::parse_outer)?,
            units: input.parse_field("units")?,
            base_quantities: input.parse_braced_field("base")?,
            quantities: input.parse_braced_field("quantities")?,
        })
    }
}

impl Parse for BaseQuantity {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            attributes: input.call(Attribute::parse_outer)?,
            name: input.parse()?,
            symbol: {
                let _ = input.parse::<Token![,]>()?;

                input.parse()?
            },
        })
    }
}

impl Parse for Quantity {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let add_mod = input
            .parse::<Option<Token![mod]>>()?
            .map_or_else(|| input.parse::<Token![use]>().map(|_| false), |_| Ok(true))?;
        let mut module: Path = input.parse()?;
        let quantity = if let Some(syn::punctuated::Pair::End(ps)) = module.segments.pop() {
            if let Some(syn::punctuated::Pair::Punctuated(end, _)) = module.segments.pop() {
                module.segments.push_value(end);
            }

            ps.ident
        } else {
            return Err(Error::new(module.span(), "expected path to quantity"));
        };

        Ok(Self { add_mod, module, quantity })
    }
}
