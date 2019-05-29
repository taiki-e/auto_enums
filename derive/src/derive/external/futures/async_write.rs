use crate::utils::*;

pub(crate) const NAME: &[&str] = &["futures::AsyncWrite"];

pub(crate) fn derive(data: &Data, items: &mut Vec<ItemImpl>) -> Result<()> {
    let io = quote!(::futures::io);

    derive_trait!(
        data,
        parse_quote!(#io::AsyncWrite)?,
        parse_quote! {
            trait AsyncWrite {
                #[inline]
                fn poll_write(
                    self: ::core::pin::Pin<&mut Self>,
                    cx: &mut ::core::task::Context<'_>,
                    buf: &[u8],
                ) -> ::core::task::Poll<::core::result::Result<usize, #io::Error>>;
                #[inline]
                fn poll_write_vectored(
                    self: ::core::pin::Pin<&mut Self>,
                    cx: &mut ::core::task::Context<'_>,
                    bufs: &[::std::io::IoSlice<'_>],
                ) -> ::core::task::Poll<::core::result::Result<usize, #io::Error>>;
                #[inline]
                fn poll_flush(
                    self: ::core::pin::Pin<&mut Self>,
                    cx: &mut ::core::task::Context<'_>,
                ) -> ::core::task::Poll<::core::result::Result<(), #io::Error>>;
                #[inline]
                fn poll_close(
                    self: ::core::pin::Pin<&mut Self>,
                    cx: &mut ::core::task::Context<'_>,
                ) -> ::core::task::Poll<::core::result::Result<(), #io::Error>>;
            }
        }?,
    )
    .map(|item| items.push(item))
}
