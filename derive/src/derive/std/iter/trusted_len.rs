use proc_macro2::TokenStream;
use quote::quote;
use smallvec::smallvec;

use crate::utils::*;

pub(crate) const NAME: &[&str] = &["TrustedLen"];

pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
    let root = std_root();
    let iter = quote!(#root::iter);

    data.impl_trait_with_capacity(
        0,
        root,
        syn::parse2(quote!(#iter::TrustedLen))?,
        smallvec![ident_call_site("Item")],
        syn::parse2(quote! {
            unsafe trait TrustedLen: #iter::Iterator {}
        })?,
    )
    .map(build)
}
