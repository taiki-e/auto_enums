use proc_macro2::TokenStream;

use crate::utils::*;

pub(crate) const NAME: &[&str] = &["Future"];

pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
    let root = std_root();

    derive_trait!(
        data,
        parse_quote!(#root::future::Future)?,
        parse_quote! {
            trait Future {
                type Output;
                #[inline]
                fn poll(
                    self: #root::pin::Pin<&mut Self>,
                    waker: &#root::task::Waker
                ) -> #root::task::Poll<Self::Output>;
            }
        }?,
    )
}
