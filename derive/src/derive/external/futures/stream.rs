use proc_macro2::TokenStream;

use crate::utils::*;

pub(crate) const NAME: &[&str] = &["futures::Stream"];

pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
    derive_trait!(
        data,
        parse_quote!(::futures::stream::Stream)?,
        parse_quote! {
            trait Stream {
                type Item;
                #[inline]
                fn poll_next(
                    self: ::core::pin::Pin<&mut Self>,
                    waker: &::core::task::Waker,
                ) -> ::core::task::Poll<::core::option::Option<Self::Item>>;
            }
        }?,
    )
}
