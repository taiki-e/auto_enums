use proc_macro2::TokenStream;
use quote::quote;

use crate::utils::*;

pub(crate) const NAME: &[&str] = &["IndexMut"];

pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
    let root = std_root();
    let ops = quote!(#root::ops);

    #[cfg(not(feature = "unsized_locals"))]
    let bounds = TokenStream::new();
    #[cfg(feature = "unsized_locals")]
    let bounds = quote!(: ?Sized);

    derive_trait!(
        data,
        Some(ident_call_site("Output")),
        parse_quote!(#ops::IndexMut)?,
        parse_quote! {
            trait IndexMut<__Idx #bounds>: #ops::Index<__Idx> {
                #[inline]
                fn index_mut(&mut self, index: __Idx) -> &mut Self::Output;
            }
        }?,
    )
}
