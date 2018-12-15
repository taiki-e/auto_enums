use proc_macro2::TokenStream;
use quote::quote;

use crate::utils::*;

pub(crate) const NAME: &[&str] = &["Future"];

pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
    let root = std_root();
    let pin = quote!(#root::pin::Pin);

    let mut impls = data.impl_trait_with_capacity(
        2,
        syn::parse2(quote!(#root::future::Future))?,
        None,
        syn::parse2(quote! {
            trait Future {
                type Output;
            }
        })?,
    )?;

    impls.push_method_pin_mut(syn::parse2(quote! {
        #[inline]
        fn poll(self: #pin<&mut Self>, lw: &#root::task::LocalWaker) -> #root::task::Poll<Self::Output>;
    })?)?;

    Ok(impls.build())
}
