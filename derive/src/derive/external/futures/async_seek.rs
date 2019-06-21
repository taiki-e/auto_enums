use crate::utils::*;

pub(crate) const NAME: &[&str] = &["futures::AsyncSeek"];

pub(crate) fn derive(data: &Data, items: &mut Vec<ItemImpl>) -> Result<()> {
    let io = quote!(::futures::io);

    derive_trait!(
        data,
        parse_quote!(#io::AsyncSeek)?,
        parse_quote! {
            trait AsyncSeek {
                #[inline]
                fn poll_seek(
                    self: ::core::pin::Pin<&mut Self>,
                    cx: &mut ::core::task::Context<'_>,
                    pos: ::std::io::SeekFrom,
                ) -> ::core::task::Poll<::std::io::Result<u64>>;
            }
        }?,
    )
    .map(|item| items.push(item))
}
