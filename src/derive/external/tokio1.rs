pub(crate) mod async_buf_read {
    use crate::derive::*;

    pub(crate) const NAME: &[&str] = &["tokio1::AsyncBufRead"];

    pub(crate) fn derive(cx: &Context, data: &Data) -> Result<TokenStream> {
        cx.needs_pin_projection();
        Ok(derive_trait(data, parse_quote!(::tokio::io::AsyncBufRead), None, parse_quote! {
            trait AsyncBufRead {
                fn poll_fill_buf<'__a>(
                    self: ::core::pin::Pin<&'__a mut Self>,
                    cx: &mut ::core::task::Context<'_>,
                ) -> ::core::task::Poll<::std::io::Result<&'__a [u8]>>;
                fn consume(self: ::core::pin::Pin<&mut Self>, amt: usize);
            }
        }))
    }
}

pub(crate) mod async_read {
    use crate::derive::*;

    pub(crate) const NAME: &[&str] = &["tokio1::AsyncRead"];

    pub(crate) fn derive(cx: &Context, data: &Data) -> Result<TokenStream> {
        cx.needs_pin_projection();
        Ok(derive_trait(data, parse_quote!(::tokio::io::AsyncRead), None, parse_quote! {
            trait AsyncRead {
                fn poll_read(
                    self: ::core::pin::Pin<&mut Self>,
                    cx: &mut ::core::task::Context<'_>,
                    buf: &mut ::tokio::io::ReadBuf<'_>,
                ) -> ::core::task::Poll<::std::io::Result<()>>;
            }
        }))
    }
}

pub(crate) mod async_seek {
    use crate::derive::*;

    pub(crate) const NAME: &[&str] = &["tokio1::AsyncSeek"];

    pub(crate) fn derive(cx: &Context, data: &Data) -> Result<TokenStream> {
        cx.needs_pin_projection();
        Ok(derive_trait(data, parse_quote!(::tokio::io::AsyncSeek), None, parse_quote! {
            trait AsyncSeek {
                fn start_seek(
                    self: ::core::pin::Pin<&mut Self>,
                    pos: ::std::io::SeekFrom,
                ) -> ::std::io::Result<()>;
                fn poll_complete(
                    self: ::core::pin::Pin<&mut Self>,
                    cx: &mut ::core::task::Context<'_>,
                ) -> ::core::task::Poll<::std::io::Result<u64>>;
            }
        }))
    }
}

pub(crate) mod async_write {
    use crate::derive::*;

    pub(crate) const NAME: &[&str] = &["tokio1::AsyncWrite"];

    pub(crate) fn derive(cx: &Context, data: &Data) -> Result<TokenStream> {
        cx.needs_pin_projection();
        Ok(derive_trait(data, parse_quote!(::tokio::io::AsyncWrite), None, parse_quote! {
            trait AsyncWrite {
                fn poll_write(
                    self: ::core::pin::Pin<&mut Self>,
                    cx: &mut ::core::task::Context<'_>,
                    buf: &[u8],
                ) -> ::core::task::Poll<::std::io::Result<usize>>;
                fn poll_flush(
                    self: ::core::pin::Pin<&mut Self>,
                    cx: &mut ::core::task::Context<'_>,
                ) -> ::core::task::Poll<::std::io::Result<()>>;
                fn poll_shutdown(
                    self: ::core::pin::Pin<&mut Self>,
                    cx: &mut ::core::task::Context<'_>,
                ) -> ::core::task::Poll<::std::io::Result<()>>;
                fn poll_write_vectored(
                    self: ::core::pin::Pin<&mut Self>,
                    cx: &mut ::core::task::Context<'_>,
                    bufs: &[::std::io::IoSlice<'_>],
                ) -> ::core::task::Poll<::std::io::Result<usize>>;
                fn is_write_vectored(&self) -> bool;
            }
        }))
    }
}
