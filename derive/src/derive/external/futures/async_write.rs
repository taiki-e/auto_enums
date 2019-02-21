use proc_macro2::TokenStream;

use crate::utils::*;

pub(crate) const NAME: &[&str] = &["futures::AsyncWrite"];

pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
    let root = std_root();
    let io = quote!(::futures::io);

    derive_trait!(
        data,
        parse_quote!(#io::AsyncWrite)?,
        parse_quote! {
            trait AsyncWrite {
                #[inline]
                fn poll_write(
                    &mut self,
                    waker: &#root::task::Waker,
                    buf: &[u8],
                ) -> #root::task::Poll<#root::result::Result<usize, #io::Error>>;
                #[inline]
                fn poll_vectored_write(
                    &mut self,
                    waker: &#root::task::Waker,
                    vec: &[&#io::IoVec],
                ) -> #root::task::Poll<#root::result::Result<usize, #io::Error>>;
                #[inline]
                fn poll_flush(
                    &mut self,
                    waker: &#root::task::Waker,
                ) -> #root::task::Poll<#root::result::Result<(), #io::Error>>;
                #[inline]
                fn poll_close(
                    &mut self,
                    waker: &#root::task::Waker,
                ) -> #root::task::Poll<#root::result::Result<(), #io::Error>>;
            }
        }?,
    )
}
