use proc_macro2::TokenStream;
use quote::quote;

use crate::utils::*;

pub(crate) const NAME: &[&str] = &["FusedIterator"];

pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
    let root = std_root();
    let iter = quote!(#root::iter);

    derive_trait_with_capacity!(
        data,
        0,
        Some(ident_call_site("Item")),
        syn::parse2(quote!(#iter::FusedIterator))?,
        syn::parse2(quote! {
            trait FusedIterator: #iter::Iterator {}
        })?
    )
}
