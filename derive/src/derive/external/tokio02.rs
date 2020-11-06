pub(crate) mod async_buf_read {
    use crate::utils::*;

    pub(crate) const NAME: &[&str] = &["tokio02::AsyncBufRead"];

    pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
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
    use crate::utils::*;

    pub(crate) const NAME: &[&str] = &["tokio02::AsyncRead"];

    pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
        Ok(derive_trait(data, parse_quote!(::tokio::io::AsyncRead), None, parse_quote! {
            trait AsyncRead {
                unsafe fn prepare_uninitialized_buffer(
                    &self,
                    buf: &mut [::core::mem::MaybeUninit<u8>],
                ) -> bool;
                fn poll_read(
                    self: ::core::pin::Pin<&mut Self>,
                    cx: &mut ::core::task::Context<'_>,
                    buf: &mut [u8],
                ) -> ::core::task::Poll<::std::io::Result<usize>>;
                // tokio02 seems does not reexport BufMut.
                // fn poll_read_buf<__B: BufMut>(
                //     self: ::core::pin::Pin<&mut Self>,
                //     cx: &mut ::core::task::Context<'_>,
                //     buf: &mut __B,
                // ) -> ::core::task::Poll<::std::io::Result<usize>>
                // where
                //     Self: Sized;
            }
        }))
    }
}

pub(crate) mod async_seek {
    use crate::utils::*;

    pub(crate) const NAME: &[&str] = &["tokio02::AsyncSeek"];

    pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
        Ok(derive_trait(data, parse_quote!(::tokio::io::AsyncSeek), None, parse_quote! {
            trait AsyncSeek {
                fn start_seek(
                    self: ::core::pin::Pin<&mut Self>,
                    cx: &mut ::core::task::Context<'_>,
                    pos: ::std::io::SeekFrom,
                ) -> ::core::task::Poll<::std::io::Result<()>>;
                fn poll_complete(
                    self: ::core::pin::Pin<&mut Self>,
                    cx: &mut ::core::task::Context<'_>,
                ) -> ::core::task::Poll<::std::io::Result<u64>>;
            }
        }))
    }
}

pub(crate) mod async_write {
    use crate::utils::*;

    pub(crate) const NAME: &[&str] = &["tokio02::AsyncWrite"];

    pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
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
                // tokio02 seems does not reexport Buf.
                // fn poll_write_buf<__B: Buf>(
                //     self: ::core::pin::Pin<&mut Self>,
                //     cx: &mut ::core::task::Context<'_>,
                //     buf: &mut __B,
                // ) -> ::core::task::Poll<::std::io::Result<usize>>
                // where
                //     Self: Sized;
            }
        }))
    }
}
