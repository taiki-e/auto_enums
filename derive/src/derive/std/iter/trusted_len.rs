use proc_macro2::TokenStream;
use quote::quote;

use crate::utils::*;

pub(crate) const NAME: &[&str] = &["TrustedLen"];

pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
    let root = std_root();
    let iter = quote!(#root::iter);

    derive_trait!(
        data,
        Some(ident_call_site("Item")),
        syn::parse2(quote!(#iter::TrustedLen))?,
        syn::parse2(quote! {
            unsafe trait TrustedLen: #iter::Iterator {}
        })?
    )
}
