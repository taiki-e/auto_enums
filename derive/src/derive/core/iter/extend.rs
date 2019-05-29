use crate::utils::*;

pub(crate) const NAME: &[&str] = &["Extend"];

pub(crate) fn derive(data: &Data, items: &mut Vec<ItemImpl>) -> Result<()> {
    derive_trait!(
        data,
        parse_quote!(::core::iter::Extend)?,
        parse_quote! {
            trait Extend<__A> {
                #[inline]
                fn extend<__T: ::core::iter::IntoIterator<Item = __A>>(&mut self, iter: __T);
            }
        }?,
    )
    .map(|item| items.push(item))
}
