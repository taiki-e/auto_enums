use proc_macro2::TokenStream;
use quote::quote;
use smallvec::SmallVec;

use crate::utils::*;

pub(crate) const NAME: &[&str] = &["fmt::Write"];

pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
    let root = std_root();
    let fmt = quote!(#root::fmt);

    data.impl_trait_with_capacity(
        3,
        root.clone(),
        syn::parse2(quote!(#fmt::Write))?,
        SmallVec::new(),
        syn::parse2(quote! {
            trait Write {
                #[inline]
                fn write_str(&mut self, s: &str) -> #fmt::Result;
                #[inline]
                fn write_char(&mut self, c: char) -> #fmt::Result;
                #[inline]
                fn write_fmt(&mut self, args: #fmt::Arguments<'_>) -> #fmt::Result;
            }
        })?,
    )
    .map(build)
}
