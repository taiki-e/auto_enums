// SPDX-License-Identifier: Apache-2.0 OR MIT

pub(crate) mod body {
    use quote::ToTokens;

    use crate::derive::*;

    pub(crate) const NAME: &[&str] = &["http_body1::Body"];

    pub(crate) fn derive(cx: &Context, data: &Data) -> Result<TokenStream> {
        cx.needs_pin_projection();

        let ident = &data.ident;
        let pin = quote!(::core::pin::Pin);
        let trait_ = parse_quote!(::http_body::Body);
        let mut impl_ = EnumImpl::from_trait(data, &trait_, None, parse_quote! {
            trait Body {
                type Data;
                type Error;
                #[inline]
                fn is_end_stream(&self) -> bool;
                #[inline]
                fn size_hint(&self) -> ::http_body::SizeHint;
            }
        })
        .build_impl();

        let poll_frame = data.variant_idents().zip(data.field_types()).map(|(v, ty)| {
            quote! {
                #ident::#v(x) => <#ty as #trait_>::poll_frame(#pin::new_unchecked(x), cx),
            }
        });
        impl_.items.push(parse_quote! {
            fn poll_frame(
                self: #pin<&mut Self>,
                cx: &mut ::core::task::Context<'_>,
            ) -> ::core::task::Poll<
                ::core::option::Option<
                    ::core::result::Result<::http_body::Frame<Self::Data>, Self::Error>,
                >,
            > {
                unsafe {
                    match self.get_unchecked_mut() { #(#poll_frame)* }
                }
            }
        });

        Ok(impl_.into_token_stream())
    }
}
