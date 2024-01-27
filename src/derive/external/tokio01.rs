// SPDX-License-Identifier: Apache-2.0 OR MIT

pub(crate) mod async_read {
    use crate::derive::*;

    pub(crate) const NAME: &[&str] = &["tokio01::AsyncRead"];

    pub(crate) fn derive(_cx: &Context, data: &Data) -> Result<TokenStream> {
        Ok(derive_trait(data, &parse_quote!(::tokio::io::AsyncRead), None, parse_quote! {
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
        }))
    }
}

pub(crate) mod async_write {
    use crate::derive::*;

    pub(crate) const NAME: &[&str] = &["tokio01::AsyncWrite"];

    pub(crate) fn derive(_cx: &Context, data: &Data) -> Result<TokenStream> {
        Ok(derive_trait(data, &parse_quote!(::tokio::io::AsyncWrite), None, parse_quote! {
            trait AsyncWrite: ::std::io::Write {
                fn poll_write(
                    &mut self,
                    buf: &[u8],
                ) -> ::tokio::prelude::Poll<usize, ::std::io::Error>;
                fn poll_flush(&mut self) -> ::tokio::prelude::Poll<(), ::std::io::Error>;
                fn shutdown(&mut self) -> ::tokio::prelude::Poll<(), ::std::io::Error>;
                // tokio01 seems does not reexport Buf.
                // fn write_buf<__B: Buf>(&mut self, buf: &mut __B) -> ::tokio::prelude::Poll<usize, ::std::io::Error>
                // where
                //     Self: Sized;
            }
        }))
    }
}
