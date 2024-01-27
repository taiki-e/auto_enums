// SPDX-License-Identifier: Apache-2.0 OR MIT

use quote::ToTokens;

use crate::derive::*;

pub(crate) const NAME: &[&str] = &["Future"];

pub(crate) fn derive(cx: &Context, data: &Data) -> Result<TokenStream> {
    cx.needs_pin_projection();

    let ident = &data.ident;
    let pin = quote!(::core::pin::Pin);
    let trait_ = parse_quote!(::core::future::Future);
    let mut impl_ = EnumImpl::from_trait(data, &trait_, None, parse_quote! {
        trait Future {
            type Output;
        }
    })
    .build_impl();

    let poll = data.variant_idents().zip(data.field_types()).map(|(v, ty)| {
        quote! {
            #ident::#v(x) => <#ty as #trait_>::poll(#pin::new_unchecked(x), cx),
        }
    });
    impl_.items.push(parse_quote! {
        #[inline]
        fn poll(
            self: #pin<&mut Self>,
            cx: &mut ::core::task::Context<'_>,
        ) -> ::core::task::Poll<Self::Output> {
            unsafe {
                match self.get_unchecked_mut() { #(#poll)* }
            }
        }
    });

    Ok(impl_.into_token_stream())
}
