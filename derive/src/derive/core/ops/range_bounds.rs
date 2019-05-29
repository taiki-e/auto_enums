use crate::utils::*;

pub(crate) const NAME: &[&str] = &["RangeBounds"];

pub(crate) fn derive(data: &Data, items: &mut Vec<ItemImpl>) -> Result<()> {
    derive_trait!(
        data,
        parse_quote!(::core::ops::RangeBounds)?,
        parse_quote! {
            trait RangeBounds<__T: ?Sized> {
                #[inline]
                fn start_bound(&self) -> ::core::ops::Bound<&__T>;
                #[inline]
                fn end_bound(&self) -> ::core::ops::Bound<&__T>;
            }
        }?,
    )
    .map(|item| items.push(item))
}
