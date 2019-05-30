use crate::utils::*;

pub(crate) const NAME: &[&str] = &["futures::AsyncRead"];

pub(crate) fn derive(data: &Data, items: &mut Vec<ItemImpl>) -> Result<()> {
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
                fn poll_read_vectored(
                    self: ::core::pin::Pin<&mut Self>,
                    cx: &mut ::core::task::Context<'_>,
                    bufs: &mut [::std::io::IoSliceMut<'_>],
                ) -> ::core::task::Poll<::core::result::Result<usize, #io::Error>>;
            }
        }?,
    )
    .map(|item| items.push(item))
}
