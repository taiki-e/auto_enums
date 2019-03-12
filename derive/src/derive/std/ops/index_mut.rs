use crate::utils::*;

pub(crate) const NAME: &[&str] = &["IndexMut"];

pub(crate) fn derive(data: &Data, stack: &mut Stack<ItemImpl>) -> Result<()> {
    #[cfg(not(feature = "unsized_locals"))]
    let bounds = quote!();
    #[cfg(feature = "unsized_locals")]
    let bounds = quote!(: ?Sized);

    derive_trait!(
        data,
        Some(ident("Output")),
        parse_quote!(::core::ops::IndexMut)?,
        parse_quote! {
            trait IndexMut<__Idx #bounds>: ::core::ops::Index<__Idx> {
                #[inline]
                fn index_mut(&mut self, index: __Idx) -> &mut Self::Output;
            }
        }?,
    )
    .map(|item| stack.push(item))
}
