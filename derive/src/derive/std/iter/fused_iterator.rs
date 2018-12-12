use proc_macro2::TokenStream;
use quote::quote;
use smallvec::smallvec;

use crate::utils::*;

pub(crate) const NAME: &[&str] = &["FusedIterator"];

pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
    let root = std_root();
    let iter = quote!(#root::iter);

    data.impl_trait_with_capacity(
        0,
        root,
        syn::parse2(quote!(#iter::FusedIterator))?,
        smallvec![ident_call_site("Item")],
        syn::parse2(quote! {
            trait FusedIterator: #iter::Iterator {}
        })?,
    )
    .map(build)
}
