use crate::utils::*;

pub(crate) const NAME: &[&str] = &["futures01::Future"];

pub(crate) fn derive(data: &Data, stack: &mut Stack<ItemImpl>) -> Result<()> {
    let crate_ = quote!(::futures);

    derive_trait!(
        data,
        parse_quote!(#crate_::future::Future)?,
        parse_quote! {
            trait Future {
                type Item;
                type Error;
                #[inline]
                fn poll(&mut self) -> #crate_::Poll<Self::Item, Self::Error>;
            }
        }?,
    )
    .map(|item| stack.push(item))
}
