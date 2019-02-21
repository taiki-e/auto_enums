use proc_macro2::TokenStream;

use crate::utils::*;

pub(crate) const NAME: &[&str] = &["futures::Sink"];

pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
    let root = std_root();

    derive_trait!(
        data,
        parse_quote!(::futures::sink::Sink)?,
        parse_quote! {
            trait Sink {
                type SinkItem;
                type SinkError;
                #[inline]
                fn poll_ready(
                    self: #root::pin::Pin<&mut Self>,
                    waker: &#root::task::Waker,
                ) -> #root::task::Poll<#root::result::Result<(), Self::SinkError>>;
                #[inline]
                fn start_send(
                    self: #root::pin::Pin<&mut Self>,
                    item: Self::SinkItem,
                ) -> #root::result::Result<(), Self::SinkError>;
                #[inline]
                fn poll_flush(
                    self: #root::pin::Pin<&mut Self>,
                    waker: &#root::task::Waker,
                ) -> #root::task::Poll<#root::result::Result<(), Self::SinkError>>;
                #[inline]
                fn poll_close(
                    self: #root::pin::Pin<&mut Self>,
                    waker: &#root::task::Waker,
                ) -> #root::task::Poll<#root::result::Result<(), Self::SinkError>>;
            }
        }?,
    )
}
