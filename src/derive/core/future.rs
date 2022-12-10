use crate::derive::*;

pub(crate) const NAME: &[&str] = &["Future"];

pub(crate) fn derive(cx: &Context, data: &Data) -> Result<TokenStream> {
    cx.needs_pin_projection();
    Ok(derive_trait(data, parse_quote!(::core::future::Future), None, parse_quote! {
        trait Future {
            type Output;
            #[inline]
            fn poll(
                self: ::core::pin::Pin<&mut Self>,
                cx: &mut ::core::task::Context<'_>,
            ) -> ::core::task::Poll<Self::Output>;
        }
    }))
}
