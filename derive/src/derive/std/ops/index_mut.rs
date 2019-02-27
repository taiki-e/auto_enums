use proc_macro2::TokenStream;

use crate::utils::*;

pub(crate) const NAME: &[&str] = &["IndexMut"];

pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
    #[cfg(not(feature = "unsized_locals"))]
    let bounds = TokenStream::new();
    #[cfg(feature = "unsized_locals")]
    let bounds = quote!(: ?Sized);

    derive_trait!(
        data,
        Some(ident_call_site("Output")),
        parse_quote!(::core::ops::IndexMut)?,
        parse_quote! {
            trait IndexMut<__Idx #bounds>: ::core::ops::Index<__Idx> {
                #[inline]
                fn index_mut(&mut self, index: __Idx) -> &mut Self::Output;
            }
        }?,
    )
}
