// SPDX-License-Identifier: Apache-2.0 OR MIT

pub(crate) mod async_buf_read {
    use crate::derive::prelude::*;

    pub(crate) const NAME: &[&str] = &["tokio02::AsyncBufRead"];

    pub(crate) fn derive(cx: &Context, data: &Data) -> Result<TokenStream> {
        cx.needs_pin_projection();

        let ident = &data.ident;
        let pin = quote!(::core::pin::Pin);
        let trait_ = parse_quote!(::tokio::io::AsyncBufRead);
        let mut impl_ = EnumImpl::from_trait(data, &trait_, None, parse_quote! {
            trait AsyncBufRead {}
        })
        .build_impl();

        let poll_fill_buf = data.variant_idents().zip(data.field_types()).map(|(v, ty)| {
            quote! {
                #ident::#v(x) => <#ty as #trait_>::poll_fill_buf(#pin::new_unchecked(x), cx),
            }
        });
        impl_.items.push(parse_quote! {
            fn poll_fill_buf(
                self: #pin<&mut Self>,
                cx: &mut ::core::task::Context<'_>,
            ) -> ::core::task::Poll<::std::io::Result<&[u8]>> {
                unsafe {
                    match self.get_unchecked_mut() { #(#poll_fill_buf)* }
                }
            }
        });

        let consume = data.variant_idents().zip(data.field_types()).map(|(v, ty)| {
            quote! {
                #ident::#v(x) => <#ty as #trait_>::consume(#pin::new_unchecked(x), amt),
            }
        });
        impl_.items.push(parse_quote! {
            fn consume(self: #pin<&mut Self>, amt: usize) {
                unsafe {
                    match self.get_unchecked_mut() { #(#consume)* }
                }
            }
        });

        Ok(impl_.into_token_stream())
    }
}

pub(crate) mod async_read {
    use crate::derive::prelude::*;

    pub(crate) const NAME: &[&str] = &["tokio02::AsyncRead"];

    pub(crate) fn derive(cx: &Context, data: &Data) -> Result<TokenStream> {
        cx.needs_pin_projection();

        let ident = &data.ident;
        let pin = quote!(::core::pin::Pin);
        let trait_ = parse_quote!(::tokio::io::AsyncRead);
        let mut impl_ = EnumImpl::from_trait(data, &trait_, None, parse_quote! {
            trait AsyncRead {
                unsafe fn prepare_uninitialized_buffer(
                    &self,
                    buf: &mut [::core::mem::MaybeUninit<u8>],
                ) -> bool;
            }
        })
        .build_impl();

        let poll_read = data.variant_idents().zip(data.field_types()).map(|(v, ty)| {
            quote! {
                #ident::#v(x) => <#ty as #trait_>::poll_read(#pin::new_unchecked(x), cx, buf),
            }
        });
        impl_.items.push(parse_quote! {
            fn poll_read(
                self: #pin<&mut Self>,
                cx: &mut ::core::task::Context<'_>,
                buf: &mut [u8],
            ) -> ::core::task::Poll<::std::io::Result<usize>> {
                unsafe {
                    match self.get_unchecked_mut() { #(#poll_read)* }
                }
            }
        });

        // tokio02 seems does not reexport BufMut.
        // fn poll_read_buf<__B: BufMut>(
        //     self: ::core::pin::Pin<&mut Self>,
        //     cx: &mut ::core::task::Context<'_>,
        //     buf: &mut __B,
        // ) -> ::core::task::Poll<::std::io::Result<usize>>
        // where
        //     Self: Sized;

        Ok(impl_.into_token_stream())
    }
}

pub(crate) mod async_seek {
    use crate::derive::prelude::*;

    pub(crate) const NAME: &[&str] = &["tokio02::AsyncSeek"];

    pub(crate) fn derive(cx: &Context, data: &Data) -> Result<TokenStream> {
        cx.needs_pin_projection();

        let ident = &data.ident;
        let pin = quote!(::core::pin::Pin);
        let trait_ = parse_quote!(::tokio::io::AsyncSeek);
        let mut impl_ = EnumImpl::from_trait(data, &trait_, None, parse_quote! {
            trait AsyncSeek {}
        })
        .build_impl();

        let start_seek = data.variant_idents().zip(data.field_types()).map(|(v, ty)| {
            quote! {
                #ident::#v(x) => <#ty as #trait_>::start_seek(#pin::new_unchecked(x), cx, pos),
            }
        });
        impl_.items.push(parse_quote! {
            fn start_seek(
                self: #pin<&mut Self>,
                cx: &mut ::core::task::Context<'_>,
                pos: ::std::io::SeekFrom,
            ) -> ::core::task::Poll<::std::io::Result<()>> {
                unsafe {
                    match self.get_unchecked_mut() { #(#start_seek)* }
                }
            }
        });

        let poll_complete = data.variant_idents().zip(data.field_types()).map(|(v, ty)| {
            quote! {
                #ident::#v(x) => <#ty as #trait_>::poll_complete(#pin::new_unchecked(x), cx),
            }
        });
        impl_.items.push(parse_quote! {
            fn poll_complete(
                self: #pin<&mut Self>,
                cx: &mut ::core::task::Context<'_>,
            ) -> ::core::task::Poll<::std::io::Result<u64>> {
                unsafe {
                    match self.get_unchecked_mut() { #(#poll_complete)* }
                }
            }
        });

        Ok(impl_.into_token_stream())
    }
}

pub(crate) mod async_write {
    use crate::derive::prelude::*;

    pub(crate) const NAME: &[&str] = &["tokio02::AsyncWrite"];

    pub(crate) fn derive(cx: &Context, data: &Data) -> Result<TokenStream> {
        cx.needs_pin_projection();

        let ident = &data.ident;
        let pin = quote!(::core::pin::Pin);
        let trait_ = parse_quote!(::tokio::io::AsyncWrite);
        let mut impl_ = EnumImpl::from_trait(data, &trait_, None, parse_quote! {
            trait AsyncWrite {}
        })
        .build_impl();

        let poll_write = data.variant_idents().zip(data.field_types()).map(|(v, ty)| {
            quote! {
                #ident::#v(x) => <#ty as #trait_>::poll_write(#pin::new_unchecked(x), cx, buf),
            }
        });
        impl_.items.push(parse_quote! {
            fn poll_write(
                self: #pin<&mut Self>,
                cx: &mut ::core::task::Context<'_>,
                buf: &[u8],
            ) -> ::core::task::Poll<::std::io::Result<usize>> {
                unsafe {
                    match self.get_unchecked_mut() { #(#poll_write)* }
                }
            }
        });

        let poll_flush = data.variant_idents().zip(data.field_types()).map(|(v, ty)| {
            quote! {
                #ident::#v(x) => <#ty as #trait_>::poll_flush(#pin::new_unchecked(x), cx),
            }
        });
        impl_.items.push(parse_quote! {
            fn poll_flush(
                self: #pin<&mut Self>,
                cx: &mut ::core::task::Context<'_>,
            ) -> ::core::task::Poll<::std::io::Result<()>> {
                unsafe {
                    match self.get_unchecked_mut() { #(#poll_flush)* }
                }
            }
        });

        let poll_shutdown = data.variant_idents().zip(data.field_types()).map(|(v, ty)| {
            quote! {
                #ident::#v(x) => <#ty as #trait_>::poll_shutdown(#pin::new_unchecked(x), cx),
            }
        });
        impl_.items.push(parse_quote! {
            fn poll_shutdown(
                self: #pin<&mut Self>,
                cx: &mut ::core::task::Context<'_>,
            ) -> ::core::task::Poll<::std::io::Result<()>> {
                unsafe {
                    match self.get_unchecked_mut() { #(#poll_shutdown)* }
                }
            }
        });

        // tokio02 seems does not reexport Buf.
        // fn poll_write_buf<__B: Buf>(
        //     self: ::core::pin::Pin<&mut Self>,
        //     cx: &mut ::core::task::Context<'_>,
        //     buf: &mut __B,
        // ) -> ::core::task::Poll<::std::io::Result<usize>>
        // where
        //     Self: Sized;

        Ok(impl_.into_token_stream())
    }
}
