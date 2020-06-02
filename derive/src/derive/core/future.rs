use crate::utils::*;

pub(crate) const NAME: &[&str] = &["Future"];

pub(crate) fn derive(data: &Data, items: &mut Vec<ItemImpl>) -> Result<()> {
    derive_trait!(data, parse_quote!(::core::future::Future)?, parse_quote! {
        trait Future {
            type Output;
            #[inline]
            fn poll(
                self: ::core::pin::Pin<&mut Self>,
                cx: &mut ::core::task::Context<'_>,
            ) -> ::core::task::Poll<Self::Output>;
        }
    }?,)
    .map(|item| items.push(item))
}
