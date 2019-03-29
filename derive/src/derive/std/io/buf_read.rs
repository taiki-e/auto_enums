use crate::utils::*;

pub(crate) const NAME: &[&str] = &["BufRead", "io::BufRead"];

pub(crate) fn derive(data: &Data, stack: &mut Stack<ItemImpl>) -> Result<()> {
    derive_trait!(
        data,
        parse_quote!(::std::io::BufRead)?,
        parse_quote! {
            trait BufRead {
                #[inline]
                fn fill_buf(&mut self) -> ::std::io::Result<&[u8]>;
                #[inline]
                fn consume(&mut self, amt: usize);
                #[inline]
                fn read_until(&mut self, byte: u8, buf: &mut ::std::vec::Vec<u8>) -> ::std::io::Result<usize>;
                #[inline]
                fn read_line(&mut self, buf: &mut ::std::string::String) -> ::std::io::Result<usize>;
            }
        }?,
    )
    .map(|item| stack.push(item))
}
