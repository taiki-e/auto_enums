use proc_macro2::TokenStream;
use quote::quote;

use crate::utils::*;

pub(crate) const NAME: &[&str] = &["Future"];

pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
    let root = std_root();
    let pin = quote!(#root::pin::Pin);

    derive_trait_with_capacity!(
        data,
        2,
        syn::parse2(quote!(#root::future::Future))?,
        syn::parse2(quote! {
            trait Future {
                type Output;
                #[inline]
                fn poll(self: #pin<&mut Self>, lw: &#root::task::LocalWaker) -> #root::task::Poll<Self::Output>;
            }
        })?
    )
}
