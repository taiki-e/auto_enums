use crate::utils::*;

pub(crate) const NAME: &[&str] = &["BufRead", "io::BufRead"];

pub(crate) fn derive(data: &Data, items: &mut Vec<ItemImpl>) -> Result<()> {
    derive_trait!(
        data,
        parse_quote!(::std::io::BufRead)?,
        parse_quote! {
            trait BufRead {
                fn fill_buf(&mut self) -> ::std::io::Result<&[u8]>;
                fn consume(&mut self, amt: usize);
                fn read_until(&mut self, byte: u8, buf: &mut ::std::vec::Vec<u8>) -> ::std::io::Result<usize>;
                fn read_line(&mut self, buf: &mut ::std::string::String) -> ::std::io::Result<usize>;
            }
        }?,
    )
    .map(|item| items.push(item))
}
