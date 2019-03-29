use crate::utils::*;

pub(crate) const NAME: &[&str] = &["Future"];

pub(crate) fn derive(data: &Data, stack: &mut Stack<ItemImpl>) -> Result<()> {
    derive_trait!(
        data,
        parse_quote!(::core::future::Future)?,
        parse_quote! {
            trait Future {
                type Output;
                #[inline]
                fn poll(
                    self: ::core::pin::Pin<&mut Self>,
                    waker: &::core::task::Waker
                ) -> ::core::task::Poll<Self::Output>;
            }
        }?,
    )
    .map(|item| stack.push(item))
}
