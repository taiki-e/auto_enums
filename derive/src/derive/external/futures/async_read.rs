use proc_macro2::TokenStream;

use crate::utils::*;

pub(crate) const NAME: &[&str] = &["futures::AsyncRead"];

pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
    let root = std_root();
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
                    lw: &#root::task::LocalWaker,
                    buf: &mut [u8],
                ) -> #root::task::Poll<#root::result::Result<usize, #io::Error>>;
                #[inline]
                fn poll_vectored_read(
                    &mut self,
                    lw: &#root::task::LocalWaker,
                    vec: &mut [&mut #io::IoVec],
                ) -> #root::task::Poll<#root::result::Result<usize, #io::Error>>;
            }
        }?,
    )
}
