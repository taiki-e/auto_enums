use crate::utils::*;

pub(crate) const NAME: &[&str] = &["fmt::Write"];

pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
    derive_trait(data, parse_quote!(::core::fmt::Write), None, parse_quote! {
        trait Write {
            #[inline]
            fn write_str(&mut self, s: &str) -> ::core::fmt::Result;
            #[inline]
            fn write_char(&mut self, c: char) -> ::core::fmt::Result;
            #[inline]
            fn write_fmt(&mut self, args: ::core::fmt::Arguments<'_>) -> ::core::fmt::Result;
        }
    })
}
