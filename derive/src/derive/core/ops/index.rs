use crate::utils::*;

pub(crate) const NAME: &[&str] = &["Index"];

pub(crate) fn derive(data: &Data, items: &mut Vec<ItemImpl>) -> Result<()> {
    derive_trait!(
        data,
        parse_quote!(::core::ops::Index)?,
        parse_quote! {
            trait Index<__Idx> {
                type Output;
                #[inline]
                fn index(&self, index: __Idx) -> &Self::Output;
            }
        }?,
    )
    .map(|item| items.push(item))
}
