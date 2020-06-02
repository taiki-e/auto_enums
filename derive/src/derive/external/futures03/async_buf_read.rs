use crate::utils::*;

pub(crate) const NAME: &[&str] = &["futures::AsyncBufRead"];

pub(crate) fn derive(data: &Data, items: &mut Vec<ItemImpl>) -> Result<()> {
    derive_trait!(data, parse_quote!(::futures::io::AsyncBufRead)?, parse_quote! {
        trait AsyncBufRead {
            #[inline]
            fn poll_fill_buf<'__a>(
                self: ::core::pin::Pin<&'__a mut Self>,
                cx: &mut ::core::task::Context<'_>,
            ) -> ::core::task::Poll<::std::io::Result<&'__a [u8]>>;
            #[inline]
            fn consume(self: ::core::pin::Pin<&mut Self>, amt: usize);
        }
    }?,)
    .map(|item| items.push(item))
}
