use proc_macro2::TokenStream;
use quote::quote;

use crate::utils::*;

pub(crate) const NAME: &[&str] = &["IndexMut"];

pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
    let root = std_root();
    let ops = quote!(#root::ops);

    #[cfg(feature = "unsized_locals")]
    let bounds = quote!(: ?Sized);
    #[cfg(not(feature = "unsized_locals"))]
    let bounds = quote!();

    derive_trait_with_capacity!(
        data,
        1,
        Some(ident_call_site("Output")),
        syn::parse2(quote!(#ops::IndexMut))?,
        syn::parse2(quote! {
            trait IndexMut<__Idx #bounds>: #ops::Index<__Idx> {
                #[inline]
                fn index_mut(&mut self, index: __Idx) -> &mut Self::Output;
            }
        })?
    )
}
