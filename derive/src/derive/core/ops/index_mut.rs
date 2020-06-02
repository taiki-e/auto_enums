use crate::utils::*;

pub(crate) const NAME: &[&str] = &["IndexMut"];

pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
    derive_trait(
        data,
        parse_quote!(::core::ops::IndexMut),
        Some(format_ident!("Output")),
        parse_quote! {
            trait IndexMut<__Idx>: ::core::ops::Index<__Idx> {
                #[inline]
                fn index_mut(&mut self, index: __Idx) -> &mut Self::Output;
            }
        },
    )
}
