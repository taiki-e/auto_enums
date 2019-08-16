use crate::utils::*;

pub(crate) const NAME: &[&str] = &["DerefMut"];

pub(crate) fn derive(data: &Data, items: &mut Vec<ItemImpl>) -> Result<()> {
    derive_trait!(
        data,
        Some(format_ident!("Target")),
        parse_quote!(::core::ops::DerefMut)?,
        parse_quote! {
            trait DerefMut: ::core::ops::Deref {
                #[inline]
                fn deref_mut(&mut self) -> &mut Self::Target;
            }
        }?,
    )
    .map(|item| items.push(item))
}
