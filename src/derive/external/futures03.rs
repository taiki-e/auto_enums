// SPDX-License-Identifier: Apache-2.0 OR MIT

pub(crate) mod async_buf_read {
    use quote::ToTokens;

    use crate::derive::*;

    pub(crate) const NAME: &[&str] = &["futures03::AsyncBufRead"];

    pub(crate) fn derive(cx: &Context, data: &Data) -> Result<TokenStream> {
        cx.needs_pin_projection();

        let ident = &data.ident;
        let pin = quote!(::core::pin::Pin);
        let trait_ = parse_quote!(::futures::io::AsyncBufRead);
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
            #[inline]
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
            #[inline]
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
    use quote::ToTokens;

    use crate::derive::*;

    pub(crate) const NAME: &[&str] = &["futures03::AsyncRead"];

    pub(crate) fn derive(cx: &Context, data: &Data) -> Result<TokenStream> {
        cx.needs_pin_projection();

        let ident = &data.ident;
        let pin = quote!(::core::pin::Pin);
        let trait_ = parse_quote!(::futures::io::AsyncRead);
        let mut impl_ = EnumImpl::from_trait(data, &trait_, None, parse_quote! {
            trait AsyncRead {}
        })
        .build_impl();

        let poll_read = data.variant_idents().zip(data.field_types()).map(|(v, ty)| {
            quote! {
                #ident::#v(x) => <#ty as #trait_>::poll_read(#pin::new_unchecked(x), cx, buf),
            }
        });
        impl_.items.push(parse_quote! {
            #[inline]
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

        let poll_read_vectored = data.variant_idents().zip(data.field_types()).map(|(v, ty)| {
            quote! {
                #ident::#v(x)
                    => <#ty as #trait_>::poll_read_vectored(#pin::new_unchecked(x), cx, bufs),
            }
        });
        impl_.items.push(parse_quote! {
            #[inline]
            fn poll_read_vectored(
                self: #pin<&mut Self>,
                cx: &mut ::core::task::Context<'_>,
                bufs: &mut [::std::io::IoSliceMut<'_>],
            ) -> ::core::task::Poll<::std::io::Result<usize>> {
                unsafe {
                    match self.get_unchecked_mut() { #(#poll_read_vectored)* }
                }
            }
        });

        Ok(impl_.into_token_stream())
    }
}

pub(crate) mod async_seek {
    use quote::ToTokens;

    use crate::derive::*;

    pub(crate) const NAME: &[&str] = &["futures03::AsyncSeek"];

    pub(crate) fn derive(cx: &Context, data: &Data) -> Result<TokenStream> {
        cx.needs_pin_projection();

        let ident = &data.ident;
        let pin = quote!(::core::pin::Pin);
        let trait_ = parse_quote!(::futures::io::AsyncSeek);
        let mut impl_ = EnumImpl::from_trait(data, &trait_, None, parse_quote! {
            trait AsyncSeek {}
        })
        .build_impl();

        let poll_seek = data.variant_idents().zip(data.field_types()).map(|(v, ty)| {
            quote! {
                #ident::#v(x) => <#ty as #trait_>::poll_seek(#pin::new_unchecked(x), cx, pos),
            }
        });
        impl_.items.push(parse_quote! {
            #[inline]
            fn poll_seek(
                self: #pin<&mut Self>,
                cx: &mut ::core::task::Context<'_>,
                pos: ::std::io::SeekFrom,
            ) -> ::core::task::Poll<::std::io::Result<u64>> {
                unsafe {
                    match self.get_unchecked_mut() { #(#poll_seek)* }
                }
            }
        });

        Ok(impl_.into_token_stream())
    }
}

pub(crate) mod async_write {
    use quote::ToTokens;

    use crate::derive::*;

    pub(crate) const NAME: &[&str] = &["futures03::AsyncWrite"];

    pub(crate) fn derive(cx: &Context, data: &Data) -> Result<TokenStream> {
        cx.needs_pin_projection();

        let ident = &data.ident;
        let pin = quote!(::core::pin::Pin);
        let trait_ = parse_quote!(::futures::io::AsyncWrite);
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
            #[inline]
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

        let poll_write_vectored = data.variant_idents().zip(data.field_types()).map(|(v, ty)| {
            quote! {
                #ident::#v(x)
                    => <#ty as #trait_>::poll_write_vectored(#pin::new_unchecked(x), cx, bufs),
            }
        });
        impl_.items.push(parse_quote! {
            #[inline]
            fn poll_write_vectored(
                self: #pin<&mut Self>,
                cx: &mut ::core::task::Context<'_>,
                bufs: &[::std::io::IoSlice<'_>],
            ) -> ::core::task::Poll<::std::io::Result<usize>> {
                unsafe {
                    match self.get_unchecked_mut() { #(#poll_write_vectored)* }
                }
            }
        });

        let poll_flush = data.variant_idents().zip(data.field_types()).map(|(v, ty)| {
            quote! {
                #ident::#v(x) => <#ty as #trait_>::poll_flush(#pin::new_unchecked(x), cx),
            }
        });
        impl_.items.push(parse_quote! {
            #[inline]
            fn poll_flush(
                self: #pin<&mut Self>,
                cx: &mut ::core::task::Context<'_>,
            ) -> ::core::task::Poll<::std::io::Result<()>> {
                unsafe {
                    match self.get_unchecked_mut() { #(#poll_flush)* }
                }
            }
        });

        let poll_close = data.variant_idents().zip(data.field_types()).map(|(v, ty)| {
            quote! {
                #ident::#v(x) => <#ty as #trait_>::poll_close(#pin::new_unchecked(x), cx),
            }
        });
        impl_.items.push(parse_quote! {
            #[inline]
            fn poll_close(
                self: #pin<&mut Self>,
                cx: &mut ::core::task::Context<'_>,
            ) -> ::core::task::Poll<::std::io::Result<()>> {
                unsafe {
                    match self.get_unchecked_mut() { #(#poll_close)* }
                }
            }
        });

        Ok(impl_.into_token_stream())
    }
}

pub(crate) mod sink {
    use quote::ToTokens;

    use crate::derive::*;

    pub(crate) const NAME: &[&str] = &["futures03::Sink"];

    pub(crate) fn derive(cx: &Context, data: &Data) -> Result<TokenStream> {
        cx.needs_pin_projection();

        let ident = &data.ident;
        let pin = quote!(::core::pin::Pin);
        let trait_ = parse_quote!(::futures::sink::Sink);
        let mut impl_ = EnumImpl::from_trait(data, &trait_, None, parse_quote! {
            trait Sink<__Item> {
                type Error;
            }
        })
        .build_impl();

        let poll_ready = data.variant_idents().zip(data.field_types()).map(|(v, ty)| {
            quote! {
                #ident::#v(x) => <#ty as #trait_<__Item>>::poll_ready(#pin::new_unchecked(x), cx),
            }
        });
        impl_.items.push(parse_quote! {
            #[inline]
            fn poll_ready(
                self: #pin<&mut Self>,
                cx: &mut ::core::task::Context<'_>,
            ) -> ::core::task::Poll<::core::result::Result<(), Self::Error>> {
                unsafe {
                    match self.get_unchecked_mut() { #(#poll_ready)* }
                }
            }
        });

        let start_send = data.variant_idents().zip(data.field_types()).map(|(v, ty)| {
            quote! {
                #ident::#v(x) => <#ty as #trait_<__Item>>::start_send(#pin::new_unchecked(x), item),
            }
        });
        impl_.items.push(parse_quote! {
            #[inline]
            fn start_send(
                self: #pin<&mut Self>,
                item: __Item,
            ) -> ::core::result::Result<(), Self::Error> {
                unsafe {
                    match self.get_unchecked_mut() { #(#start_send)* }
                }
            }
        });

        let poll_flush = data.variant_idents().zip(data.field_types()).map(|(v, ty)| {
            quote! {
                #ident::#v(x) => <#ty as #trait_<__Item>>::poll_flush(#pin::new_unchecked(x), cx),
            }
        });
        impl_.items.push(parse_quote! {
            #[inline]
            fn poll_flush(
                self: #pin<&mut Self>,
                cx: &mut ::core::task::Context<'_>,
            ) -> ::core::task::Poll<::core::result::Result<(), Self::Error>> {
                unsafe {
                    match self.get_unchecked_mut() { #(#poll_flush)* }
                }
            }
        });

        let poll_close = data.variant_idents().zip(data.field_types()).map(|(v, ty)| {
            quote! {
                #ident::#v(x) => <#ty as #trait_<__Item>>::poll_close(#pin::new_unchecked(x), cx),
            }
        });
        impl_.items.push(parse_quote! {
            #[inline]
            fn poll_close(
                self: #pin<&mut Self>,
                cx: &mut ::core::task::Context<'_>,
            ) -> ::core::task::Poll<::core::result::Result<(), Self::Error>> {
                unsafe {
                    match self.get_unchecked_mut() { #(#poll_close)* }
                }
            }
        });

        Ok(impl_.into_token_stream())
    }
}

pub(crate) mod stream {
    use quote::ToTokens;

    use crate::derive::*;

    pub(crate) const NAME: &[&str] = &["futures03::Stream"];

    pub(crate) fn derive(cx: &Context, data: &Data) -> Result<TokenStream> {
        cx.needs_pin_projection();

        let ident = &data.ident;
        let pin = quote!(::core::pin::Pin);
        let trait_ = parse_quote!(::futures::stream::Stream);
        let mut impl_ = EnumImpl::from_trait(data, &trait_, None, parse_quote! {
            trait Stream {
                type Item;
                #[inline]
                fn size_hint(&self) -> (usize, ::core::option::Option<usize>);
            }
        })
        .build_impl();

        let poll_next = data.variant_idents().zip(data.field_types()).map(|(v, ty)| {
            quote! {
                #ident::#v(x) => <#ty as #trait_>::poll_next(#pin::new_unchecked(x), cx),
            }
        });
        impl_.items.push(parse_quote! {
            #[inline]
            fn poll_next(
                self: #pin<&mut Self>,
                cx: &mut ::core::task::Context<'_>,
            ) -> ::core::task::Poll<::core::option::Option<Self::Item>> {
                unsafe {
                    match self.get_unchecked_mut() { #(#poll_next)* }
                }
            }
        });

        Ok(impl_.into_token_stream())
    }
}
