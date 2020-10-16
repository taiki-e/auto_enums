use crate::utils::*;

pub(crate) const NAME: &[&str] = &["tokio03::AsyncRead"];

pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
    derive_trait(data, parse_quote!(::tokio::io::AsyncRead), None, parse_quote! {
        trait AsyncRead {
            fn poll_read(
                self: ::core::pin::Pin<&mut Self>,
                cx: &mut ::core::task::Context<'_>,
                buf: &mut ::tokio::io::ReadBuf<'_>,
            ) -> ::core::task::Poll<::std::io::Result<()>>;
        }
    })
}
