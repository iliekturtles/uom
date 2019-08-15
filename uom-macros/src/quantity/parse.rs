use super::*;
use crate::parse::ParseStreamExt;
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::{Attribute, Error, Token};

impl Parse for Quantity {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            name_attributes: input.call(Attribute::parse_outer)?,
            name: input.parse_field("quantity")?,
            description: input.parse_field("description")?,
            dimension_attributes: input.call(Attribute::parse_outer)?,
            system: {
                let _ = input.expect_field("dimension")?;
                let _ = input.parse::<Token![:]>()?;

                input.parse()?
            },
            dimensions: {
                let _ = input.parse::<Token![<]>()?;
                let dimensions = Punctuated::<Ident, Token![,]>::parse_separated_nonempty(input)?;
                let _ = input.parse::<Token![>]>()?;
                let _ = input.parse::<Token![;]>()?;

                dimensions
            },
            kind: input.parse_optional_field("kind")?,
            units: input.parse_braced_field("units")?,
        })
    }
}

impl Parse for Unit {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let attributes = input.call(Attribute::parse_outer)?;
        let name = input.parse()?;
        let _ = input.parse::<Token![:]>()?;
        let properties = Punctuated::<Expr, Token![,]>::parse_separated_nonempty(input)?;
        let has_constant = match properties.len() {
            4 => false, // coefficient, abbreviation, singular, plural.
            5 => true,  // coefficient, constant, abbreviation, singular, plural.
            n => {
                return Err(Error::new(
                    properties.span(),
                    format!(
                        "unexpected number of unit properties ({})\n\
                         expected unit: coefficient, (constant,)? abbreviation, singular, plural;",
                        n
                    ),
                ));
            }
        };
        let mut properties = properties.into_iter();
        let coefficient = properties.next().unwrap();
        let constant = if has_constant { Some(properties.next().unwrap()) } else { None };
        let abbreviation = properties.next().unwrap();
        let singular = properties.next().unwrap();
        let plural = properties.next().unwrap();

        Ok(Self { attributes, name, coefficient, constant, abbreviation, singular, plural })
    }
}
