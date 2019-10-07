use crate::utils::*;

pub(crate) const NAME: &[&str] = &["ExactSizeIterator"];

pub(crate) fn derive(data: &Data, items: &mut Vec<ItemImpl>) -> Result<()> {
    // TODO: When `exact_size_is_empty` stabilized, add `is_empty` conditionally.

    derive_trait!(
        data,
        Some(format_ident!("Item")),
        parse_quote!(::core::iter::ExactSizeIterator)?,
        parse_quote! {
            trait ExactSizeIterator: ::core::iter::Iterator {
                #[inline]
                fn len(&self) -> usize;
            }
        }?,
    )
    .map(|item| items.push(item))
}
