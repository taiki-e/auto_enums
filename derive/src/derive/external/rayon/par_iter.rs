use proc_macro2::TokenStream;
use quote::quote;

use crate::utils::*;

pub(crate) const NAME: &[&str] = &["rayon::ParallelIterator"];

pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
    let root = std_root();
    let iter = quote!(::rayon::iter);

    derive_trait!(
        data,
        syn::parse2(quote!(#iter::ParallelIterator))?,
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
        })?
    )
}
