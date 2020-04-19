use crate::utils::*;

pub(crate) const NAME: &[&str] = &["futures01::Future"];

pub(crate) fn derive(data: &Data, items: &mut Vec<ItemImpl>) -> Result<()> {
    let crate_ = quote!(::futures);

    derive_trait!(
        data,
        parse_quote!(#crate_::future::Future)?,
        parse_quote! {
            trait Future {
                type Item;
                type Error;
                fn poll(&mut self) -> #crate_::Poll<Self::Item, Self::Error>;
            }
        }?,
    )
    .map(|item| items.push(item))
}
