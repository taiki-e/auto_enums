// SPDX-License-Identifier: Apache-2.0 OR MIT

pub(crate) mod future {
    use crate::derive::*;

    pub(crate) const NAME: &[&str] = &["futures01::Future"];

    pub(crate) fn derive(_cx: &Context, data: &Data) -> Result<TokenStream> {
        Ok(derive_trait(data, &parse_quote!(::futures::future::Future), None, parse_quote! {
            trait Future {
                type Item;
                type Error;
                #[inline]
                fn poll(&mut self) -> ::futures::Poll<Self::Item, Self::Error>;
            }
        }))
    }
}

pub(crate) mod stream {
    use crate::derive::*;

    pub(crate) const NAME: &[&str] = &["futures01::Stream"];

    pub(crate) fn derive(_cx: &Context, data: &Data) -> Result<TokenStream> {
        Ok(derive_trait(data, &parse_quote!(::futures::stream::Stream), None, parse_quote! {
            trait Stream {
                type Item;
                type Error;
                #[inline]
                fn poll(
                    &mut self,
                ) -> ::futures::Poll<::core::option::Option<Self::Item>, Self::Error>;
            }
        }))
    }
}

pub(crate) mod sink {
    use crate::derive::*;

    pub(crate) const NAME: &[&str] = &["futures01::Sink"];

    pub(crate) fn derive(_cx: &Context, data: &Data) -> Result<TokenStream> {
        Ok(derive_trait(data, &parse_quote!(::futures::sink::Sink), None, parse_quote! {
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
        }))
    }
}
