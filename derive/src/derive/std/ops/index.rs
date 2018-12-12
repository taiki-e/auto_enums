use proc_macro2::TokenStream;
use quote::quote;
use smallvec::SmallVec;

use crate::utils::*;

pub(crate) const NAME: &[&str] = &["Index"];

pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
    let root = std_root();
    let ops = quote!(#root::ops);

    #[cfg(feature = "unsized_locals")]
    let bounds = quote!(: ?Sized);
    #[cfg(not(feature = "unsized_locals"))]
    let bounds = quote!();

    data.impl_trait_with_capacity(
        2,
        root,
        syn::parse2(quote!(#ops::Index))?,
        SmallVec::new(),
        syn::parse2(quote! {
            trait Index<__Idx #bounds> {
                type Output;
                #[inline]
                fn index(&self, index: __Idx) -> &Self::Output;
            }
        })?,
    )
    .map(build)
}
