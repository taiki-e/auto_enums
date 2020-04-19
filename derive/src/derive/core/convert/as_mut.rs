use crate::utils::*;

pub(crate) const NAME: &[&str] = &["AsMut"];

pub(crate) fn derive(data: &Data, items: &mut Vec<ItemImpl>) -> Result<()> {
    derive_trait!(
        data,
        parse_quote!(::core::convert::AsMut)?,
        parse_quote! {
            trait AsMut<__T: ?Sized> {
                fn as_mut(&mut self) -> &mut __T;
            }
        }?,
    )
    .map(|item| items.push(item))
}
