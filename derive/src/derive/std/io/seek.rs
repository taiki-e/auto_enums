use crate::utils::*;

pub(crate) const NAME: &[&str] = &["Seek", "io::Seek"];

pub(crate) fn derive(data: &Data, stack: &mut Stack<ItemImpl>) -> Result<()> {
    let io = quote!(::std::io);

    derive_trait!(
        data,
        parse_quote!(#io::Seek)?,
        parse_quote! {
            trait Seek {
                #[inline]
                fn seek(&mut self, pos: #io::SeekFrom) -> #io::Result<u64>;
            }
        }?,
    )
    .map(|item| stack.push(item))
}
