use crate::utils::*;

pub(crate) const NAME: &[&str] = &["futures::AsyncSeek"];

pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
    derive_trait(data, parse_quote!(::futures::io::AsyncSeek), None, parse_quote! {
        trait AsyncSeek {
            #[inline]
            fn poll_seek(
                self: ::core::pin::Pin<&mut Self>,
                cx: &mut ::core::task::Context<'_>,
                pos: ::std::io::SeekFrom,
            ) -> ::core::task::Poll<::std::io::Result<u64>>;
        }
    })
}
