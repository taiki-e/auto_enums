use proc_macro2::TokenStream;
use quote::quote;

use crate::utils::*;

pub(crate) const NAME: &[&str] = &["Index"];

pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
    let root = std_root();
    let ops = quote!(#root::ops);

    #[cfg(feature = "unsized_locals")]
    let bounds = quote!(: ?Sized);
    #[cfg(not(feature = "unsized_locals"))]
    let bounds = quote!();

    derive_trait_with_capacity!(
        data,
        2,
        syn::parse2(quote!(#ops::Index))?,
        syn::parse2(quote! {
            trait Index<__Idx #bounds> {
                type Output;
                #[inline]
                fn index(&self, index: __Idx) -> &Self::Output;
            }
        })?
    )
}
