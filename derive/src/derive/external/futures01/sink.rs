use proc_macro2::TokenStream;
use quote::quote;
use smallvec::SmallVec;

use crate::utils::*;

pub(crate) const NAME: &[&str] = &["futures01::Sink"];

pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
    let crate_ = quote!(::futures);

    data.impl_trait_with_capacity(
        5,
        std_root(),
        syn::parse2(quote!(#crate_::sink::Sink))?,
        SmallVec::new(),
        syn::parse2(quote! {
            trait Sink {
                type SinkItem;
                type SinkError;
                #[inline]
                fn start_send(&mut self, item: Self::SinkItem) -> #crate_::StartSend<Self::SinkItem, Self::SinkError>;
                #[inline]
                fn poll_complete(&mut self) -> #crate_::Poll<(), Self::SinkError>;
                #[inline]
                fn close(&mut self) -> #crate_::Poll<(), Self::SinkError>;
            }
        })?,
    )
    .map(build)
}

