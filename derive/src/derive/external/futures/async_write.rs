use crate::utils::*;

pub(crate) const NAME: &[&str] = &["futures::AsyncWrite"];

pub(crate) fn derive(data: &Data, stack: &mut Stack<ItemImpl>) -> Result<()> {
    let io = quote!(::futures::io);

    derive_trait!(
        data,
        parse_quote!(#io::AsyncWrite)?,
        parse_quote! {
            trait AsyncWrite {
                #[inline]
                fn poll_write(
                    &mut self,
                    waker: &::core::task::Waker,
                    buf: &[u8],
                ) -> ::core::task::Poll<::core::result::Result<usize, #io::Error>>;
                #[inline]
                fn poll_vectored_write(
                    &mut self,
                    waker: &::core::task::Waker,
                    vec: &[&#io::IoVec],
                ) -> ::core::task::Poll<::core::result::Result<usize, #io::Error>>;
                #[inline]
                fn poll_flush(
                    &mut self,
                    waker: &::core::task::Waker,
                ) -> ::core::task::Poll<::core::result::Result<(), #io::Error>>;
                #[inline]
                fn poll_close(
                    &mut self,
                    waker: &::core::task::Waker,
                ) -> ::core::task::Poll<::core::result::Result<(), #io::Error>>;
            }
        }?,
    )
    .map(|item| stack.push(item))
}
