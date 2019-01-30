use proc_macro2::TokenStream;

use crate::utils::*;

pub(crate) const NAME: &[&str] = &["Write", "io::Write"];

pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
    let root = std_root();
    let io = quote!(#root::io);

    derive_trait!(
        data,
        parse_quote!(#io::Write)?,
        parse_quote! {
            trait Write {
                #[inline]
                fn write(&mut self, buf: &[u8]) -> #io::Result<usize>;
                #[inline]
                fn flush(&mut self) -> #io::Result<()>;
                #[inline]
                fn write_all(&mut self, buf: &[u8]) -> #io::Result<()>;
                #[inline]
                fn write_fmt(&mut self, fmt: #root::fmt::Arguments) -> #io::Result<()>;
            }
        }?,
    )
}
