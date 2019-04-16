use crate::utils::*;

pub(crate) const NAME: &[&str] = &["futures::AsyncRead"];

pub(crate) fn derive(data: &Data, stack: &mut Stack<ItemImpl>) -> Result<()> {
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
                    self: ::core::pin::Pin<&mut Self>,
                    cx: &mut ::core::task::Context<'_>,
                    buf: &mut [u8],
                ) -> ::core::task::Poll<::core::result::Result<usize, #io::Error>>;
                #[inline]
                fn poll_vectored_read(
                    self: ::core::pin::Pin<&mut Self>,
                    cx: &mut ::core::task::Context<'_>,
                    vec: &mut [&mut #io::IoVec],
                ) -> ::core::task::Poll<::core::result::Result<usize, #io::Error>>;
            }
        }?,
    )
    .map(|item| stack.push(item))
}
