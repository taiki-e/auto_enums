use crate::utils::*;

pub(crate) const NAME: &[&str] = &["Seek", "io::Seek"];

pub(crate) fn derive(data: &Data, items: &mut Vec<ItemImpl>) -> Result<()> {
    derive_trait!(
        data,
        parse_quote!(::std::io::Seek)?,
        parse_quote! {
            trait Seek {
                #[inline]
                fn seek(&mut self, pos: ::std::io::SeekFrom) -> ::std::io::Result<u64>;
            }
        }?,
    )
    .map(|item| items.push(item))
}
