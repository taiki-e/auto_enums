use crate::utils::*;

pub(crate) const NAME: &[&str] = &["tokio02::AsyncSeek"];

pub(crate) fn derive(data: &Data, items: &mut Vec<ItemImpl>) -> Result<()> {
    derive_trait!(
        data,
        parse_quote!(::tokio::io::AsyncSeek)?,
        parse_quote! {
            trait AsyncSeek {
                fn start_seek(
                    self: ::core::pin::Pin<&mut Self>,
                    cx: &mut ::core::task::Context<'_>,
                    pos: ::std::io::SeekFrom,
                ) -> ::core::task::Poll<::std::io::Result<()>>;
                fn poll_complete(
                    self: ::core::pin::Pin<&mut Self>,
                    cx: &mut ::core::task::Context<'_>,
                ) -> ::core::task::Poll<::std::io::Result<u64>>;
            }
        }?,
    )
    .map(|item| items.push(item))
}
