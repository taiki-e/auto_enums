use proc_macro2::TokenStream;
use quote::quote;

use crate::utils::*;

pub(crate) const NAME: &[&str] = &["ExactSizeIterator"];

pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
    #[cfg(feature = "exact_size_is_empty")]
    const CAPACITY: usize = 2;
    #[cfg(not(feature = "exact_size_is_empty"))]
    const CAPACITY: usize = 1;

    let root = std_root();
    let iter = quote!(#root::iter);

    #[allow(unused_mut)]
    let mut impls = data.impl_trait_with_capacity(
        CAPACITY,
        syn::parse2(quote!(#iter::ExactSizeIterator))?,
        Some(ident_call_site("Item")),
        syn::parse2(quote! {
            trait ExactSizeIterator: #iter::Iterator {
                #[inline]
                fn len(&self) -> usize;
            }
        })?,
    )?;

    #[cfg(feature = "exact_size_is_empty")]
    impls.push_method(syn::parse2(quote! {
        #[inline]
        fn is_empty(&self) -> bool;
    })?)?;

    Ok(impls.build())
}
