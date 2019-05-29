use crate::utils::*;

pub(crate) const NAME: &[&str] = &["AsRef"];

pub(crate) fn derive(data: &Data, items: &mut Vec<ItemImpl>) -> Result<()> {
    derive_trait!(
        data,
        parse_quote!(::core::convert::AsRef)?,
        parse_quote! {
            trait AsRef<__T: ?Sized> {
                #[inline]
                fn as_ref(&self) -> &__T;
            }
        }?,
    )
    .map(|item| items.push(item))
}
