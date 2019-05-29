use crate::utils::*;

pub(crate) const NAME: &[&str] = &["FusedIterator"];

pub(crate) fn derive(data: &Data, items: &mut Vec<ItemImpl>) -> Result<()> {
    derive_trait!(
        data,
        Some(ident("Item")),
        parse_quote!(::core::iter::FusedIterator)?,
        parse_quote! {
            trait FusedIterator: ::core::iter::Iterator {}
        }?,
    )
    .map(|item| items.push(item))
}
