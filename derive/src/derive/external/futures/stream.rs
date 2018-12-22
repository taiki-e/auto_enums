use proc_macro2::TokenStream;
use quote::quote;

use crate::utils::*;

pub(crate) const NAME: &[&str] = &["futures::Stream"];

pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
    let root = std_root();
    let pin = quote!(#root::pin::Pin);

    derive_trait_with_capacity!(
        data,
        2,
        syn::parse2(quote!(::futures::stream::Stream))?,
        syn::parse2(quote! {
            trait Stream {
                type Item;
                #[inline]
                fn poll_next(self: #pin<&mut Self>, lw: &#root::task::LocalWaker) -> #root::task::Poll<#root::option::Option<Self::Item>>;
            }
        })?
    )
}
