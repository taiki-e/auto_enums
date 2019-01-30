use proc_macro2::TokenStream;

use crate::utils::*;

pub(crate) const NAME: &[&str] = &["Read", "io::Read"];

pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
    let root = std_root();
    let io = quote!(#root::io);

    #[cfg(not(feature = "read_initializer"))]
    let initializer = TokenStream::new();
    #[cfg(feature = "read_initializer")]
    let initializer = quote! {
        #[inline]
        unsafe fn initializer(&self) -> #io::Initializer;
    };

    derive_trait!(
        data,
        parse_quote!(#io::Read)?,
        parse_quote! {
            trait Iterator {
                #[inline]
                fn read(&mut self, buf: &mut [u8]) -> #io::Result<usize>;
                #[inline]
                fn read_to_end(&mut self, buf: &mut #root::vec::Vec<u8>) -> #io::Result<usize>;
                #[inline]
                fn read_to_string(&mut self, buf: &mut #root::string::String) -> #io::Result<usize>;
                #[inline]
                fn read_exact(&mut self, buf: &mut [u8]) -> #io::Result<()>;
                #initializer
            }
        }?,
    )
}
