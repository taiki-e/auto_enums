use crate::utils::*;

pub(crate) const NAME: &[&str] = &["ExactSizeIterator"];

pub(crate) fn derive(data: &Data, stack: &mut Stack<ItemImpl>) -> Result<()> {
    #[cfg(not(feature = "exact_size_is_empty"))]
    let is_empty = quote!();
    #[cfg(feature = "exact_size_is_empty")]
    let is_empty = quote! {
        #[inline]
        fn is_empty(&self) -> bool;
    };

    derive_trait!(
        data,
        Some(ident_call_site("Item")),
        parse_quote!(::core::iter::ExactSizeIterator)?,
        parse_quote! {
            trait ExactSizeIterator: ::core::iter::Iterator {
                #[inline]
                fn len(&self) -> usize;
                #is_empty
            }
        }?,
    )
    .map(|item| stack.push(item))
}
