use crate::utils::*;

pub(crate) const NAME: &[&str] = &["futures::AsyncWrite"];

pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
    derive_trait(data, parse_quote!(::futures::io::AsyncWrite), None, parse_quote! {
        trait AsyncWrite {
            #[inline]
            fn poll_write(
                self: ::core::pin::Pin<&mut Self>,
                cx: &mut ::core::task::Context<'_>,
                buf: &[u8],
            ) -> ::core::task::Poll<::std::io::Result<usize>>;
            #[inline]
            fn poll_write_vectored(
                self: ::core::pin::Pin<&mut Self>,
                cx: &mut ::core::task::Context<'_>,
                bufs: &[::std::io::IoSlice<'_>],
            ) -> ::core::task::Poll<::std::io::Result<usize>>;
            #[inline]
            fn poll_flush(
                self: ::core::pin::Pin<&mut Self>,
                cx: &mut ::core::task::Context<'_>,
            ) -> ::core::task::Poll<::std::io::Result<()>>;
            #[inline]
            fn poll_close(
                self: ::core::pin::Pin<&mut Self>,
                cx: &mut ::core::task::Context<'_>,
            ) -> ::core::task::Poll<::std::io::Result<()>>;
        }
    })
}
