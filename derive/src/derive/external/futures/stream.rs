use proc_macro2::TokenStream;

use crate::utils::*;

pub(crate) const NAME: &[&str] = &["futures::Stream"];

pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
    let root = std_root();

    derive_trait!(
        data,
        parse_quote!(::futures::stream::Stream)?,
        parse_quote! {
            trait Stream {
                type Item;
                #[inline]
                fn poll_next(
                    self: #root::pin::Pin<&mut Self>,
                    lw: &#root::task::LocalWaker,
                ) -> #root::task::Poll<#root::option::Option<Self::Item>>;
            }
        }?,
    )
}
