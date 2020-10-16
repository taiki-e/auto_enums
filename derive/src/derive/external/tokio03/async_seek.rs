use crate::utils::*;

pub(crate) const NAME: &[&str] = &["tokio03::AsyncSeek"];

pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
    derive_trait(data, parse_quote!(::tokio::io::AsyncSeek), None, parse_quote! {
        trait AsyncSeek {
            fn start_seek(
                self: ::core::pin::Pin<&mut Self>,
                pos: ::std::io::SeekFrom,
            ) -> ::std::io::Result<()>;
            fn poll_complete(
                self: ::core::pin::Pin<&mut Self>,
                cx: &mut ::core::task::Context<'_>,
            ) -> ::core::task::Poll<::std::io::Result<u64>>;
        }
    })
}
