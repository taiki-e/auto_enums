use crate::utils::*;

pub(crate) const NAME: &[&str] = &["Deref"];

pub(crate) fn derive(data: &Data, items: &mut Vec<ItemImpl>) -> Result<()> {
    derive_trait!(
        data,
        parse_quote!(::core::ops::Deref)?,
        parse_quote! {
            trait Deref {
                type Target;
                #[inline]
                fn deref(&self) -> &Self::Target;
            }
        }?,
    )
    .map(|item| items.push(item))
}
