use crate::utils::*;

pub(crate) const NAME: &[&str] = &["IndexMut"];

pub(crate) fn derive(data: &Data, items: &mut Vec<ItemImpl>) -> Result<()> {
    derive_trait!(
        data,
        Some(format_ident!("Output")),
        parse_quote!(::core::ops::IndexMut)?,
        parse_quote! {
            trait IndexMut<__Idx>: ::core::ops::Index<__Idx> {
                fn index_mut(&mut self, index: __Idx) -> &mut Self::Output;
            }
        }?,
    )
    .map(|item| items.push(item))
}
