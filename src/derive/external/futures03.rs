pub(crate) mod async_buf_read {
    use crate::derive::*;

    pub(crate) const NAME: &[&str] = &["futures03::AsyncBufRead"];

    pub(crate) fn derive(cx: &Context, data: &Data) -> Result<TokenStream> {
        cx.needs_pin_projection();
        Ok(derive_trait(data, parse_quote!(::futures::io::AsyncBufRead), None, parse_quote! {
            trait AsyncBufRead {
                #[inline]
                fn poll_fill_buf<'__a>(
                    self: ::core::pin::Pin<&'__a mut Self>,
                    cx: &mut ::core::task::Context<'_>,
                ) -> ::core::task::Poll<::std::io::Result<&'__a [u8]>>;
                #[inline]
                fn consume(self: ::core::pin::Pin<&mut Self>, amt: usize);
            }
        }))
    }
}

pub(crate) mod async_read {
    use crate::derive::*;

    pub(crate) const NAME: &[&str] = &["futures03::AsyncRead"];

    pub(crate) fn derive(cx: &Context, data: &Data) -> Result<TokenStream> {
        cx.needs_pin_projection();
        Ok(derive_trait(data, parse_quote!(::futures::io::AsyncRead), None, parse_quote! {
            trait AsyncRead {
                #[inline]
                fn poll_read(
                    self: ::core::pin::Pin<&mut Self>,
                    cx: &mut ::core::task::Context<'_>,
                    buf: &mut [u8],
                ) -> ::core::task::Poll<::std::io::Result<usize>>;
                #[inline]
                fn poll_read_vectored(
                    self: ::core::pin::Pin<&mut Self>,
                    cx: &mut ::core::task::Context<'_>,
                    bufs: &mut [::std::io::IoSliceMut<'_>],
                ) -> ::core::task::Poll<::std::io::Result<usize>>;
            }
        }))
    }
}

pub(crate) mod async_seek {
    use crate::derive::*;

    pub(crate) const NAME: &[&str] = &["futures03::AsyncSeek"];

    pub(crate) fn derive(cx: &Context, data: &Data) -> Result<TokenStream> {
        cx.needs_pin_projection();
        Ok(derive_trait(data, parse_quote!(::futures::io::AsyncSeek), None, parse_quote! {
            trait AsyncSeek {
                #[inline]
                fn poll_seek(
                    self: ::core::pin::Pin<&mut Self>,
                    cx: &mut ::core::task::Context<'_>,
                    pos: ::std::io::SeekFrom,
                ) -> ::core::task::Poll<::std::io::Result<u64>>;
            }
        }))
    }
}

pub(crate) mod async_write {
    use crate::derive::*;

    pub(crate) const NAME: &[&str] = &["futures03::AsyncWrite"];

    pub(crate) fn derive(cx: &Context, data: &Data) -> Result<TokenStream> {
        cx.needs_pin_projection();
        Ok(derive_trait(data, parse_quote!(::futures::io::AsyncWrite), None, parse_quote! {
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
        }))
    }
}

pub(crate) mod sink {
    use crate::derive::*;

    pub(crate) const NAME: &[&str] = &["futures03::Sink"];

    pub(crate) fn derive(cx: &Context, data: &Data) -> Result<TokenStream> {
        cx.needs_pin_projection();
        Ok(derive_trait(data, parse_quote!(::futures::sink::Sink), None, parse_quote! {
            trait Sink<Item> {
                type Error;
                #[inline]
                fn poll_ready(
                    self: ::core::pin::Pin<&mut Self>,
                    cx: &mut ::core::task::Context<'_>,
                ) -> ::core::task::Poll<::core::result::Result<(), Self::Error>>;
                #[inline]
                fn start_send(
                    self: ::core::pin::Pin<&mut Self>,
                    item: Item,
                ) -> ::core::result::Result<(), Self::Error>;
                #[inline]
                fn poll_flush(
                    self: ::core::pin::Pin<&mut Self>,
                    cx: &mut ::core::task::Context<'_>,
                ) -> ::core::task::Poll<::core::result::Result<(), Self::Error>>;
                #[inline]
                fn poll_close(
                    self: ::core::pin::Pin<&mut Self>,
                    cx: &mut ::core::task::Context<'_>,
                ) -> ::core::task::Poll<::core::result::Result<(), Self::Error>>;
            }
        }))
    }
}

pub(crate) mod stream {
    use crate::derive::*;

    pub(crate) const NAME: &[&str] = &["futures03::Stream"];

    pub(crate) fn derive(cx: &Context, data: &Data) -> Result<TokenStream> {
        cx.needs_pin_projection();
        Ok(derive_trait(data, parse_quote!(::futures::stream::Stream), None, parse_quote! {
            trait Stream {
                type Item;
                #[inline]
                fn poll_next(
                    self: ::core::pin::Pin<&mut Self>,
                    cx: &mut ::core::task::Context<'_>,
                ) -> ::core::task::Poll<::core::option::Option<Self::Item>>;
                #[inline]
                fn size_hint(&self) -> (usize, ::core::option::Option<usize>);
            }
        }))
    }
}
