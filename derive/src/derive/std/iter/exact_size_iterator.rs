use proc_macro2::TokenStream;
use quote::quote;

use crate::utils::*;

pub(crate) const NAME: &[&str] = &["ExactSizeIterator"];

pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
    let root = std_root();
    let iter = quote!(#root::iter);

    #[cfg(not(feature = "exact_size_is_empty"))]
    let is_empty = TokenStream::new();
    #[cfg(feature = "exact_size_is_empty")]
    let is_empty = quote! {
        #[inline]
        fn is_empty(&self) -> bool;
    };

    derive_trait!(
        data,
        Some(ident_call_site("Item")),
        syn::parse2(quote!(#iter::ExactSizeIterator))?,
        syn::parse2(quote! {
            trait ExactSizeIterator: #iter::Iterator {
                #[inline]
                fn len(&self) -> usize;
                #is_empty
            }
        })?
    )
}
