use crate::utils::*;

pub(crate) const NAME: &[&str] = &["tokio01::AsyncWrite"];

pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
    derive_trait(data, parse_quote!(::tokio::io::AsyncWrite), None, parse_quote! {
        trait AsyncWrite: ::std::io::Write {
            fn poll_write(&mut self, buf: &[u8]) -> ::tokio::prelude::Poll<usize, ::std::io::Error>;
            fn poll_flush(&mut self) -> ::tokio::prelude::Poll<(), ::std::io::Error>;
            fn shutdown(&mut self) -> ::tokio::prelude::Poll<(), ::std::io::Error>;
            // tokio01 seems does not reexport Buf.
            // fn write_buf<__B: Buf>(&mut self, buf: &mut __B) -> ::tokio::prelude::Poll<usize, ::std::io::Error>
            // where
            //     Self: Sized;
        }
    })
}
