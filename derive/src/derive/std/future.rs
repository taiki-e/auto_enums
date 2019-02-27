use proc_macro2::TokenStream;

use crate::utils::*;

pub(crate) const NAME: &[&str] = &["Future"];

pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
    derive_trait!(
        data,
        parse_quote!(::core::future::Future)?,
        parse_quote! {
            trait Future {
                type Output;
                #[inline]
                fn poll(
                    self: ::core::pin::Pin<&mut Self>,
                    waker: &::core::task::Waker
                ) -> ::core::task::Poll<Self::Output>;
            }
        }?,
    )
}
