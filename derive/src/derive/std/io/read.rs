use crate::utils::*;

pub(crate) const NAME: &[&str] = &["Read", "io::Read"];

pub(crate) fn derive(data: &Data, items: &mut Vec<ItemImpl>) -> Result<()> {
    #[cfg(not(stable_1_36))]
    let vectored = quote!();
    #[cfg(stable_1_36)]
    let vectored = quote! {
        #[inline]
        fn read_vectored(&mut self, bufs: &mut [::std::io::IoSliceMut<'_>]) -> ::std::io::Result<usize>;
    };

    #[cfg(not(feature = "read_initializer"))]
    let initializer = quote!();
    #[cfg(feature = "read_initializer")]
    let initializer = quote! {
        #[inline]
        unsafe fn initializer(&self) -> ::std::io::Initializer;
    };

    derive_trait!(
        data,
        parse_quote!(::std::io::Read)?,
        parse_quote! {
            trait Read {
                #[inline]
                fn read(&mut self, buf: &mut [u8]) -> ::std::io::Result<usize>;
                #[inline]
                fn read_to_end(&mut self, buf: &mut ::std::vec::Vec<u8>) -> ::std::io::Result<usize>;
                #[inline]
                fn read_to_string(&mut self, buf: &mut ::std::string::String) -> ::std::io::Result<usize>;
                #[inline]
                fn read_exact(&mut self, buf: &mut [u8]) -> ::std::io::Result<()>;
                #vectored
                #initializer
            }
        }?,
    )
    .map(|item| items.push(item))
}
