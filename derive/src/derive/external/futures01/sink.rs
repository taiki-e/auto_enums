use crate::utils::*;

pub(crate) const NAME: &[&str] = &["futures01::Sink"];

pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
    derive_trait(data, parse_quote!(::futures::sink::Sink), None, parse_quote! {
        trait Sink {
            type SinkItem;
            type SinkError;
            #[inline]
            fn start_send(
                &mut self,
                item: Self::SinkItem,
            ) -> ::futures::StartSend<Self::SinkItem, Self::SinkError>;
            #[inline]
            fn poll_complete(&mut self) -> ::futures::Poll<(), Self::SinkError>;
            #[inline]
            fn close(&mut self) -> ::futures::Poll<(), Self::SinkError>;
        }
    })
}
