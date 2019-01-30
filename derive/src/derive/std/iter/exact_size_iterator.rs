use proc_macro2::TokenStream;

use crate::utils::*;

pub(crate) const NAME: &[&str] = &["ExactSizeIterator"];

pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
    let root = std_root();

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
        parse_quote!(#root::iter::ExactSizeIterator)?,
        parse_quote! {
            trait ExactSizeIterator: #root::iter::Iterator {
                #[inline]
                fn len(&self) -> usize;
                #is_empty
            }
        }?,
    )
}
