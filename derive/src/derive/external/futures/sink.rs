use proc_macro2::TokenStream;

use crate::utils::*;

pub(crate) const NAME: &[&str] = &["futures::Sink"];

pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
    derive_trait!(
        data,
        parse_quote!(::futures::sink::Sink)?,
        parse_quote! {
            trait Sink {
                type SinkItem;
                type SinkError;
                #[inline]
                fn poll_ready(
                    self: ::core::pin::Pin<&mut Self>,
                    waker: &::core::task::Waker,
                ) -> ::core::task::Poll<::core::result::Result<(), Self::SinkError>>;
                #[inline]
                fn start_send(
                    self: ::core::pin::Pin<&mut Self>,
                    item: Self::SinkItem,
                ) -> ::core::result::Result<(), Self::SinkError>;
                #[inline]
                fn poll_flush(
                    self: ::core::pin::Pin<&mut Self>,
                    waker: &::core::task::Waker,
                ) -> ::core::task::Poll<::core::result::Result<(), Self::SinkError>>;
                #[inline]
                fn poll_close(
                    self: ::core::pin::Pin<&mut Self>,
                    waker: &::core::task::Waker,
                ) -> ::core::task::Poll<::core::result::Result<(), Self::SinkError>>;
            }
        }?,
    )
}
