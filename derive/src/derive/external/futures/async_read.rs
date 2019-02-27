use proc_macro2::TokenStream;

use crate::utils::*;

pub(crate) const NAME: &[&str] = &["futures::AsyncRead"];

pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
    let io = quote!(::futures::io);

    derive_trait!(
        data,
        parse_quote!(#io::AsyncRead)?,
        parse_quote! {
            trait AsyncRead {
                #[inline]
                unsafe fn initializer(&self) -> #io::Initializer;
                #[inline]
                fn poll_read(
                    &mut self,
                    waker: &::core::task::Waker,
                    buf: &mut [u8],
                ) -> ::core::task::Poll<::core::result::Result<usize, #io::Error>>;
                #[inline]
                fn poll_vectored_read(
                    &mut self,
                    waker: &::core::task::Waker,
                    vec: &mut [&mut #io::IoVec],
                ) -> ::core::task::Poll<::core::result::Result<usize, #io::Error>>;
            }
        }?,
    )
}
