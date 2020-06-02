use crate::utils::*;

pub(crate) const NAME: &[&str] = &["futures::AsyncRead"];

pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
    derive_trait(data, parse_quote!(::futures::io::AsyncRead), None, parse_quote! {
        trait AsyncRead {
            #[inline]
            fn poll_read(
                self: ::core::pin::Pin<&mut Self>,
                cx: &mut ::core::task::Context<'_>,
                buf: &mut [u8],
            ) -> ::core::task::Poll<::std::io::Result<usize>>;
            #[inline]
            fn poll_read_vectored(
                self: ::core::pin::Pin<&mut Self>,
                cx: &mut ::core::task::Context<'_>,
                bufs: &mut [::std::io::IoSliceMut<'_>],
            ) -> ::core::task::Poll<::std::io::Result<usize>>;
        }
    })
}
