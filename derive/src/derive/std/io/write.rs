use crate::utils::*;

pub(crate) const NAME: &[&str] = &["Write", "io::Write"];

pub(crate) fn derive(data: &Data, stack: &mut Stack<ItemImpl>) -> Result<()> {
    #[cfg(not(feature = "iovec"))]
    let vectored = quote!();
    #[cfg(feature = "iovec")]
    let vectored = quote! {
        #[inline]
        fn write_vectored(&mut self, bufs: &[::std::io::IoVec<'_>]) -> ::std::io::Result<usize>;
    };

    derive_trait!(
        data,
        parse_quote!(::std::io::Write)?,
        parse_quote! {
            trait Write {
                #[inline]
                fn write(&mut self, buf: &[u8]) -> ::std::io::Result<usize>;
                #[inline]
                fn flush(&mut self) -> ::std::io::Result<()>;
                #[inline]
                fn write_all(&mut self, buf: &[u8]) -> ::std::io::Result<()>;
                #[inline]
                fn write_fmt(&mut self, fmt: ::std::fmt::Arguments<'_>) -> ::std::io::Result<()>;
                #vectored
            }
        }?,
    )
    .map(|item| stack.push(item))
}
