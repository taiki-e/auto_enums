use proc_macro2::TokenStream;
use quote::quote;

use crate::utils::*;

pub(crate) const NAME: &[&str] = &["FusedIterator"];

pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
    let root = std_root();
    let iter = quote!(#root::iter);

    derive_trait!(
        data,
        Some(ident_call_site("Item")),
        parse_quote!(#iter::FusedIterator)?,
        parse_quote! {
            trait FusedIterator: #iter::Iterator {}
        }?,
    )
}
