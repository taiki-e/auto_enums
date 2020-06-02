use crate::utils::*;

pub(crate) const NAME: &[&str] = &["tokio01::AsyncRead"];

pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
    derive_trait(data, parse_quote!(::tokio::io::AsyncRead), None, parse_quote! {
        trait AsyncRead: ::std::io::Read {
            unsafe fn prepare_uninitialized_buffer(&self, buf: &mut [u8]) -> bool;
            fn poll_read(
                &mut self,
                buf: &mut [u8],
            ) -> ::tokio::prelude::Poll<usize, ::std::io::Error>;

            // tokio01 seems does not reexport BufMut.
            // fn read_buf<__B: BufMut>(
            //     &mut self,
            //     buf: &mut __B,
            // ) -> ::tokio::prelude::Poll<usize, ::std::io::Error>
            // where
            //     Self: Sized;
        }
    })
}
