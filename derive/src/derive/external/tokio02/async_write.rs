use crate::utils::*;

pub(crate) const NAME: &[&str] = &["tokio02::AsyncWrite"];

pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
    derive_trait(data, parse_quote!(::tokio::io::AsyncWrite), None, parse_quote! {
        trait AsyncWrite {
            fn poll_write(
                self: ::core::pin::Pin<&mut Self>,
                cx: &mut ::core::task::Context<'_>,
                buf: &[u8],
            ) -> ::core::task::Poll<::std::io::Result<usize>>;
            // fn poll_write_vectored(
            //     self: ::core::pin::Pin<&mut Self>,
            //     cx: &mut ::core::task::Context<'_>,
            //     bufs: &[::std::io::IoSlice<'_>],
            // ) -> ::core::task::Poll<::std::io::Result<usize>>;
            fn poll_flush(
                self: ::core::pin::Pin<&mut Self>,
                cx: &mut ::core::task::Context<'_>,
            ) -> ::core::task::Poll<::std::io::Result<()>>;
            fn poll_shutdown(
                self: ::core::pin::Pin<&mut Self>,
                cx: &mut ::core::task::Context<'_>,
            ) -> ::core::task::Poll<::std::io::Result<()>>;
            // tokio02 seems does not reexport Buf.
            // fn poll_write_buf<__B: Buf>(
            //     self: ::core::pin::Pin<&mut Self>,
            //     cx: &mut ::core::task::Context<'_>,
            //     buf: &mut __B,
            // ) -> ::core::task::Poll<::std::io::Result<usize>>
            // where
            //     Self: Sized;
        }
    })
}
