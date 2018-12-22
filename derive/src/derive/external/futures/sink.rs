use proc_macro2::TokenStream;
use quote::quote;

use crate::utils::*;

pub(crate) const NAME: &[&str] = &["futures::Sink"];

pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
    let root = std_root();
    let pin = quote!(#root::pin::Pin);

    derive_trait_with_capacity!(
        data,
        6,
        syn::parse2(quote!(::futures::sink::Sink))?,
        syn::parse2(quote! {
            trait Sink {
                type SinkItem;
                type SinkError;
                #[inline]
                fn poll_ready(self: #pin<&mut Self>, lw: &#root::task::LocalWaker) -> #root::task::Poll<#root::result::Result<(), Self::SinkError>>;
                #[inline]
                fn start_send(self: #pin<&mut Self>, item: Self::SinkItem) -> #root::result::Result<(), Self::SinkError>;
                #[inline]
                fn poll_flush(self: #pin<&mut Self>, lw: &#root::task::LocalWaker) -> #root::task::Poll<#root::result::Result<(), Self::SinkError>>;
                #[inline]
                fn poll_close(self: #pin<&mut Self>, lw: &#root::task::LocalWaker) -> #root::task::Poll<#root::result::Result<(), Self::SinkError>>;
            }
        })?
    )
}
