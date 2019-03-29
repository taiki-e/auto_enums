use crate::utils::*;

pub(crate) const NAME: &[&str] = &["IndexMut"];

pub(crate) fn derive(data: &Data, stack: &mut Stack<ItemImpl>) -> Result<()> {
    derive_trait!(
        data,
        Some(ident("Output")),
        parse_quote!(::core::ops::IndexMut)?,
        parse_quote! {
            trait IndexMut<__Idx>: ::core::ops::Index<__Idx> {
                #[inline]
                fn index_mut(&mut self, index: __Idx) -> &mut Self::Output;
            }
        }?,
    )
    .map(|item| stack.push(item))
}
