use proc_macro2::TokenStream;
use quote::quote;
use smallvec::SmallVec;

use crate::utils::*;

pub(crate) const NAME: &[&str] = &["RangeBounds"];

pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
    let root = std_root();
    let ops = quote!(#root::ops);

    data.impl_trait_with_capacity(
        2,
        root,
        syn::parse2(quote!(#ops::RangeBounds))?,
        SmallVec::new(),
        syn::parse2(quote! {
            trait RangeBounds<__T: ?Sized> {
                #[inline]
                fn start_bound(&self) -> #ops::Bound<&__T>;
                #[inline]
                fn end_bound(&self) -> #ops::Bound<&__T>;
            }
        })?,
    )
    .map(build)
}
