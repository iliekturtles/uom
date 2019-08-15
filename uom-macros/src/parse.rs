use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{braced, Error, Ident, Result, Token};

/// Extension methods for `ParseStream<'a>`.
pub(crate) trait ParseStreamExt {
    /// Expect the next token to be an `Ident` with the given name.
    fn expect_field(self, ident: &str) -> Result<()>;

    /// Parse a field: `<ident>: <value>;` where the first token must be an `Ident` with the given
    /// name.
    fn parse_field<T: Parse>(self, ident: &str) -> Result<T>;

    /// Parse a braced field: `<ident>: { <punctuated values> }` where the first token must be an
    /// `Ident` with the given name.
    fn parse_braced_field<T: Parse, P: Parse>(self, ident: &str) -> Result<Punctuated<T, P>>;
}

impl<'a> ParseStreamExt for ParseStream<'a> {
    fn expect_field(self, ident: &str) -> Result<()> {
        let token = self.parse::<Ident>()?;

        if token == ident {
            Ok(())
        } else {
            Err(Error::new(token.span(), format!("expected `{}`", ident)))
        }
    }

    fn parse_field<T: Parse>(self, ident: &str) -> Result<T> {
        let _ = self.expect_field(ident)?;
        let _ = self.parse::<Token![:]>()?;
        let field = self.parse()?;
        let _ = self.parse::<Token![;]>()?;

        Ok(field)
    }

    fn parse_braced_field<T: Parse, P: Parse>(self, ident: &str) -> Result<Punctuated<T, P>> {
        let content;

        self.expect_field(ident)?;
        braced!(content in self);

        content.parse_terminated(T::parse)
    }
}
