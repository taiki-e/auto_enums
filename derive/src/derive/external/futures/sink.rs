use crate::utils::*;

pub(crate) const NAME: &[&str] = &["futures::Sink"];

pub(crate) fn derive(data: &Data, items: &mut Vec<ItemImpl>) -> Result<()> {
    derive_trait!(
        data,
        parse_quote!(::futures::sink::Sink)?,
        parse_quote! {
            trait Sink<Item> {
                type SinkError;
                #[inline]
                fn poll_ready(
                    self: ::core::pin::Pin<&mut Self>,
                    cx: &mut ::core::task::Context<'_>,
                ) -> ::core::task::Poll<::core::result::Result<(), Self::SinkError>>;
                #[inline]
                fn start_send(
                    self: ::core::pin::Pin<&mut Self>,
                    item: Item,
                ) -> ::core::result::Result<(), Self::SinkError>;
                #[inline]
                fn poll_flush(
                    self: ::core::pin::Pin<&mut Self>,
                    cx: &mut ::core::task::Context<'_>,
                ) -> ::core::task::Poll<::core::result::Result<(), Self::SinkError>>;
                #[inline]
                fn poll_close(
                    self: ::core::pin::Pin<&mut Self>,
                    cx: &mut ::core::task::Context<'_>,
                ) -> ::core::task::Poll<::core::result::Result<(), Self::SinkError>>;
            }
        }?,
    )
    .map(|item| items.push(item))
}
