use proc_macro2::TokenStream;
use quote::quote;
use smallvec::SmallVec;

use crate::utils::*;

pub(crate) const NAME: &[&str] = &["rayon::ParallelIterator"];

pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
    let root = std_root();
    let iter = quote!(::rayon::iter);

    data.impl_trait_with_capacity(
        0,
        root.clone(),
        syn::parse2(quote!(#iter::ParallelIterator))?,
        SmallVec::new(),
        syn::parse2(quote! {
            trait ParallelIterator {
                type Item;
                #[inline]
                fn drive_unindexed<__C>(self, consumer: __C) -> __C::Result
                where
                    __C: #iter::plumbing::UnindexedConsumer<Self::Item>;
                #[inline]
                fn opt_len(&self) -> #root::option::Option<usize>;
            }
        })?,
    )
    .map(build)
}
