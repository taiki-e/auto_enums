use crate::utils::*;

pub(crate) const NAME: &[&str] = &["Read", "io::Read"];

pub(crate) fn derive(data: &Data, items: &mut Vec<ItemImpl>) -> Result<()> {
    #[cfg(not(stable_1_36))]
    let vectored = quote!();
    #[cfg(stable_1_36)]
    let vectored = quote! {
        fn read_vectored(&mut self, bufs: &mut [::std::io::IoSliceMut<'_>]) -> ::std::io::Result<usize>;
    };

    // TODO: When `read_initializer` stabilized, add `initializer` conditionally.

    derive_trait!(
        data,
        parse_quote!(::std::io::Read)?,
        parse_quote! {
            trait Read {
                fn read(&mut self, buf: &mut [u8]) -> ::std::io::Result<usize>;
                fn read_to_end(&mut self, buf: &mut ::std::vec::Vec<u8>) -> ::std::io::Result<usize>;
                fn read_to_string(&mut self, buf: &mut ::std::string::String) -> ::std::io::Result<usize>;
                fn read_exact(&mut self, buf: &mut [u8]) -> ::std::io::Result<()>;
                #vectored
            }
        }?,
    )
    .map(|item| items.push(item))
}
