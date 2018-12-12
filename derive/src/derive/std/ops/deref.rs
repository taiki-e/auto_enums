use proc_macro2::TokenStream;
use quote::quote;
use smallvec::SmallVec;

use crate::utils::*;

pub(crate) const NAME: &[&str] = &["Deref"];

pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
    let root = std_root();
    let ops = quote!(#root::ops);

    data.impl_trait_with_capacity(
        2,
        root,
        syn::parse2(quote!(#ops::Deref))?,
        SmallVec::new(),
        syn::parse2(quote! {
            trait Deref {
                type Target;
                #[inline]
                fn deref(&self) -> &Self::Target;
            }
        })?,
    )
    .map(build)
}
