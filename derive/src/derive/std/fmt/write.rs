use proc_macro2::TokenStream;
use quote::quote;

use crate::utils::*;

pub(crate) const NAME: &[&str] = &["fmt::Write"];

pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
    let root = std_root();
    let fmt = quote!(#root::fmt);

    derive_trait_with_capacity!(
        data,
        3,
        syn::parse2(quote!(#fmt::Write))?,
        syn::parse2(quote! {
            trait Write {
                #[inline]
                fn write_str(&mut self, s: &str) -> #fmt::Result;
                #[inline]
                fn write_char(&mut self, c: char) -> #fmt::Result;
                #[inline]
                fn write_fmt(&mut self, args: #fmt::Arguments<'_>) -> #fmt::Result;
            }
        })?
    )
}
