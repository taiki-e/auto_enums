use proc_macro2::TokenStream;
use quote::quote;
use smallvec::SmallVec;

use crate::utils::*;

pub(crate) const NAME: &[&str] = &["futures::Sink"];

pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
    let root = std_root();
    let pin = quote!(#root::pin::Pin);

    let mut impls = data.impl_trait_with_capacity(
        6,
        root.clone(),
        syn::parse2(quote!(::futures::sink::Sink))?,
        SmallVec::new(),
        syn::parse2(quote! {
            trait Sink {
                type SinkItem;
                type SinkError;
            }
        })?,
    )?;

    impls.push_method_pin_mut(syn::parse2(quote! {
        #[inline]
        fn poll_ready(self: #pin<&mut Self>, lw: &#root::task::LocalWaker) -> #root::task::Poll<#root::result::Result<(), Self::SinkError>>;
    })?)?;
    impls.push_method_pin_mut(syn::parse2(quote! {
        #[inline]
        fn start_send(self: #pin<&mut Self>, item: Self::SinkItem) -> #root::result::Result<(), Self::SinkError>;
    })?)?;
    impls.push_method_pin_mut(syn::parse2(quote! {
        #[inline]
        fn poll_flush(self: #pin<&mut Self>, lw: &#root::task::LocalWaker) -> #root::task::Poll<#root::result::Result<(), Self::SinkError>>;
    })?)?;
    impls.push_method_pin_mut(syn::parse2(quote! {
        #[inline]
        fn poll_close(self: #pin<&mut Self>, lw: &#root::task::LocalWaker) -> #root::task::Poll<#root::result::Result<(), Self::SinkError>>;
    })?)?;

    Ok(impls.build())
}
