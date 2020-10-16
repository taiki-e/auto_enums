use crate::utils::*;

pub(crate) const NAME: &[&str] = &["tokio02::AsyncRead"];

pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
    derive_trait(data, parse_quote!(::tokio::io::AsyncRead), None, parse_quote! {
        trait AsyncRead {
            unsafe fn prepare_uninitialized_buffer(
                &self,
                buf: &mut [::core::mem::MaybeUninit<u8>],
            ) -> bool;
            fn poll_read(
                self: ::core::pin::Pin<&mut Self>,
                cx: &mut ::core::task::Context<'_>,
                buf: &mut [u8],
            ) -> ::core::task::Poll<::std::io::Result<usize>>;
            // tokio02 seems does not reexport BufMut.
            // fn poll_read_buf<__B: BufMut>(
            //     self: ::core::pin::Pin<&mut Self>,
            //     cx: &mut ::core::task::Context<'_>,
            //     buf: &mut __B,
            // ) -> ::core::task::Poll<::std::io::Result<usize>>
            // where
            //     Self: Sized;
        }
    })
}
