use crate::utils::*;

pub(crate) const NAME: &[&str] = &["TrustedLen"];

pub(crate) fn derive(data: &Data, items: &mut Vec<ItemImpl>) -> Result<()> {
    derive_trait!(
        data,
        Some(ident("Item")),
        parse_quote!(::core::iter::TrustedLen)?,
        parse_quote! {
            unsafe trait TrustedLen: ::core::iter::Iterator {}
        }?,
    )
    .map(|item| items.push(item))
}
