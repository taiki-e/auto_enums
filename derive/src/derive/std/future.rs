use proc_macro2::TokenStream;
use quote::quote;

use crate::utils::*;

pub(crate) const NAME: &[&str] = &["Future"];

pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
    let root = std_root();
    let pin = quote!(#root::pin::Pin);

    derive_trait!(
        data,
        parse_quote!(#root::future::Future)?,
        parse_quote! {
            trait Future {
                type Output;
                #[inline]
                fn poll(self: #pin<&mut Self>, lw: &#root::task::LocalWaker) -> #root::task::Poll<Self::Output>;
            }
        }?,
    )
}
