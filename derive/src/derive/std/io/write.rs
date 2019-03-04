use crate::utils::*;

pub(crate) const NAME: &[&str] = &["Write", "io::Write"];

pub(crate) fn derive(data: &Data, stack: &mut Stack<ItemImpl>) -> Result<()> {
    let io = quote!(::std::io);

    #[cfg(not(feature = "iovec"))]
    let vectored = quote!();
    #[cfg(feature = "iovec")]
    let vectored = quote! {
        #[inline]
        fn write_vectored(&mut self, bufs: &[#io::IoVec<'_>]) -> #io::Result<usize>;
    };

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
                fn write_fmt(&mut self, fmt: ::std::fmt::Arguments<'_>) -> #io::Result<()>;
                #vectored
            }
        }?,
    )
    .map(|item| stack.push(item))
}
