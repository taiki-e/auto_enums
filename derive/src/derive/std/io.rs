pub(crate) mod read {
    use crate::utils::*;

    pub(crate) const NAME: &[&str] = &["Read", "io::Read"];

    pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
        #[cfg(not(stable_1_36))]
        let vectored = quote!();
        #[cfg(stable_1_36)]
        let vectored = quote! {
            #[inline]
            fn read_vectored(
                &mut self,
                bufs: &mut [::std::io::IoSliceMut<'_>],
            ) -> ::std::io::Result<usize>;
        };

        // TODO: When `read_initializer` or alternative stabilized, add that conditionally.

        derive_trait(data, parse_quote!(::std::io::Read), None, parse_quote! {
            trait Read {
                #[inline]
                fn read(&mut self, buf: &mut [u8]) -> ::std::io::Result<usize>;
                #[inline]
                fn read_to_end(
                    &mut self,
                    buf: &mut ::std::vec::Vec<u8>,
                ) -> ::std::io::Result<usize>;
                #[inline]
                fn read_to_string(
                    &mut self,
                    buf: &mut ::std::string::String,
                ) -> ::std::io::Result<usize>;
                #[inline]
                fn read_exact(&mut self, buf: &mut [u8]) -> ::std::io::Result<()>;
                #vectored
            }
        })
    }
}

pub(crate) mod buf_read {
    use crate::utils::*;

    pub(crate) const NAME: &[&str] = &["BufRead", "io::BufRead"];

    pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
        derive_trait(data, parse_quote!(::std::io::BufRead), None, parse_quote! {
            trait BufRead {
                #[inline]
                fn fill_buf(&mut self) -> ::std::io::Result<&[u8]>;
                #[inline]
                fn consume(&mut self, amt: usize);
                #[inline]
                fn read_until(
                    &mut self,
                    byte: u8, buf: &mut ::std::vec::Vec<u8>,
                ) -> ::std::io::Result<usize>;
                #[inline]
                fn read_line(
                    &mut self,
                    buf: &mut ::std::string::String,
                ) -> ::std::io::Result<usize>;
            }
        })
    }
}

pub(crate) mod seek {
    use crate::utils::*;

    pub(crate) const NAME: &[&str] = &["Seek", "io::Seek"];

    pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
        derive_trait(data, parse_quote!(::std::io::Seek), None, parse_quote! {
            trait Seek {
                #[inline]
                fn seek(&mut self, pos: ::std::io::SeekFrom) -> ::std::io::Result<u64>;
            }
        })
    }
}

pub(crate) mod write {
    use crate::utils::*;

    pub(crate) const NAME: &[&str] = &["Write", "io::Write"];

    pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
        #[cfg(not(stable_1_36))]
        let vectored = quote!();
        #[cfg(stable_1_36)]
        let vectored = quote! {
            #[inline]
            fn write_vectored(
                &mut self,
                bufs: &[::std::io::IoSlice<'_>],
            ) -> ::std::io::Result<usize>;
        };

        derive_trait(data, parse_quote!(::std::io::Write), None, parse_quote! {
            trait Write {
                #[inline]
                fn write(&mut self, buf: &[u8]) -> ::std::io::Result<usize>;
                #[inline]
                fn flush(&mut self) -> ::std::io::Result<()>;
                #[inline]
                fn write_all(&mut self, buf: &[u8]) -> ::std::io::Result<()>;
                #[inline]
                fn write_fmt(
                    &mut self,
                    fmt: ::std::fmt::Arguments<'_>,
                ) -> ::std::io::Result<()>;
                #vectored
            }
        })
    }
}
