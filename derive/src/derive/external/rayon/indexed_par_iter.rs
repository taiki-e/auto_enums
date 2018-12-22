use proc_macro2::TokenStream;
use quote::quote;

use crate::utils::*;

pub(crate) const NAME: &[&str] = &["rayon::IndexedParallelIterator"];

pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
    let iter = quote!(::rayon::iter);

    derive_trait_with_capacity!(
        data,
        0,
        Some(ident_call_site("Item")),
        syn::parse2(quote!(#iter::IndexedParallelIterator))?,
        syn::parse2(quote! {
            trait IndexedParallelIterator: #iter::ParallelIterator {
                #[inline]
                fn drive<__C>(self, consumer: __C) -> __C::Result
                where
                    __C: #iter::plumbing::Consumer<Self::Item>;
                #[inline]
                fn len(&self) -> usize;
                #[inline]
                fn with_producer<__CB>(self, callback: __CB) -> __CB::Output
                where
                    __CB: #iter::plumbing::ProducerCallback<Self::Item>;
            }
        })?
    )
}
